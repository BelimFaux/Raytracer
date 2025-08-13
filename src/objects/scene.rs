use crate::math::{max, Color, Ray};

use super::{
    surface::{Intersection, Surface},
    Camera, Light,
};

#[derive(Debug)]
struct Animated {
    total_frames: usize,
    curr_frame: usize,
    fps: u16,
}

/// Struct to hold all data belonging to a single scene
#[derive(Debug)]
pub struct Scene {
    output: String,
    background_color: Color,
    samples: u32,
    camera: Camera,
    lights: Vec<Light>,
    surfaces: Vec<Surface>,
    animated: Animated,
}

impl Scene {
    /// Create a new scene
    #[must_use]
    pub fn new(
        output: String,
        background_color: Color,
        camera: Camera,
        lights: Vec<Light>,
        surfaces: Vec<Surface>,
    ) -> Scene {
        Scene {
            output,
            background_color,
            samples: 0,
            camera,
            lights,
            surfaces,
            animated: Animated {
                total_frames: 1,
                curr_frame: 1,
                fps: 1,
            },
        }
    }

    /// Add the number of samples for the scene
    /// Setting this to any number other than 0 will enable super-sampling
    pub fn add_samples(&mut self, samples: u32) {
        self.samples = samples;
    }

    /// Set the scene to have an animation with the specified number of frames and fps
    pub fn set_animation(&mut self, frames: usize, fps: u16) {
        self.animated.total_frames = frames;
        self.animated.fps = fps;
    }

    /// Return a reference to the output file name
    #[must_use]
    pub fn get_output(&self) -> &str {
        &self.output
    }

    #[must_use]
    pub fn is_animated(&self) -> bool {
        self.animated.total_frames > 1
    }

    #[must_use]
    pub fn get_frames(&self) -> usize {
        self.animated.total_frames
    }

    #[must_use]
    pub fn get_fps(&self) -> u16 {
        self.animated.fps
    }

    /// change the scene to the next frame
    /// might change the properties of some objects
    pub fn next_frame(&mut self) {
        self.animated.curr_frame += 1;
        #[allow(clippy::cast_precision_loss)]
        let w = self.animated.curr_frame as f32 / self.animated.total_frames as f32;
        self.surfaces.iter_mut().for_each(|s| s.frame_perc(w));
    }

    /// Return the dimensions of the image
    #[must_use]
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.camera.get_dimensions()
    }

    /// Boolean test if a ray intersects any surface in the scene
    fn intersects_any(&self, with: &Ray) -> bool {
        self.surfaces
            .iter()
            .any(|surface| surface.has_intersection(with))
    }

    /// Find the closest intersection of a ray with any surface in the scene
    /// Returns None if no surface intersects with the ray
    fn closest_intersection(&self, with: &Ray) -> Option<Intersection<'_>> {
        self.surfaces
            .iter()
            // map each sphere to it's intersection with the ray if it exists
            .filter_map(|surface| surface.intersection(with))
            // sort the intersections by 't'
            .min_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).expect("t shouldn't be NaN"))
    }

    /// Calculate the color of an intersection
    /// iterates over all lights and sums up their color at the intersection, if they are in los of
    /// the intersection point
    fn intersection_color(&self, intersect: &Intersection, ray: &Ray) -> Color {
        self.lights
            .iter()
            // filter lights whose shadow ray intersects with any surfaces in the scene
            .filter(|light| {
                light
                    .shadow_ray(&intersect.point)
                    .is_none_or(|ray| !self.intersects_any(&ray))
            })
            // calculate the color for each light
            .map(|light| intersect.get_color(light, ray))
            // sum them together
            .reduce(|lhs, rhs| lhs + rhs)
            // if there was no light in sight, the object is black
            .unwrap_or(Color::zero())
    }

    /// Recursively ray trace a ray shot into the Scene
    /// `depth` should be the allowed maximum depth, and will be _decreased_ with every iteration
    fn recursive_trace(&self, ray: &Ray, depth: u32) -> Color {
        match self.closest_intersection(ray) {
            Some(intersection) => {
                let color = self.intersection_color(&intersection, ray);
                let mut reflected_color = Color::zero();
                let mut refracted_color = Color::zero();
                if depth == 0 {
                    return color;
                }
                if intersection.get_reflectance() > 0. {
                    let reflected_ray = intersection.reflected_ray(ray);
                    reflected_color = self.recursive_trace(&reflected_ray, depth - 1);
                }
                if intersection.get_transmittance() > 0. {
                    let refracted_ray = intersection.refracted_ray(ray);
                    refracted_color = self.recursive_trace(&refracted_ray, depth - 1);
                }
                color
                    * max(
                        1. - intersection.get_reflectance() - intersection.get_transmittance(),
                        0.0,
                    )
                    + reflected_color * intersection.get_reflectance()
                    + refracted_color * intersection.get_transmittance()
            }
            None => self.background_color,
        }
    }

    /// trace the pixel with super-sampling
    /// will panic if `samples` is 0 (0 samples doesn't really make sense, does it?)
    #[allow(clippy::cast_precision_loss)]
    fn ssaa_trace_pixel(&self, u: u32, v: u32) -> Color {
        let mut final_color = Color::zero();
        for _ in 0..self.samples {
            let ray = self.camera.get_sample_ray_through(u, v);
            final_color += self.recursive_trace(&ray, self.camera.get_max_bounces());
        }

        final_color / self.samples as f32
    }

    /// ray trace a pixel
    /// get the camera ray and test the closest intersection with any object
    /// then perform lighting calculations at the closest intersection
    #[must_use]
    pub fn trace_pixel(&self, u: u32, v: u32) -> Color {
        if self.samples != 0 {
            return self.ssaa_trace_pixel(u, v);
        }
        let ray = self.camera.get_ray_through(u, v);

        self.recursive_trace(&ray, self.camera.get_max_bounces())
    }
}
