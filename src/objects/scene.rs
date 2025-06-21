use crate::math::{max, Color, Ray};

use super::{
    surface::{Intersection, Surface},
    Camera, Light,
};

/// Struct to hold all data belonging to a single scene
#[derive(Debug)]
pub struct Scene {
    output: String,
    background_color: Color,
    camera: Camera,
    lights: Vec<Light>,
    surfaces: Vec<Surface>,
}

impl Scene {
    /// Create a new scene
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
            camera,
            lights,
            surfaces,
        }
    }

    /// Return a reference to the output file name
    pub fn get_output(&self) -> &str {
        &self.output
    }

    /// Return the dimensions of the image
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
    fn closest_intersection(&self, with: &Ray) -> Option<Intersection> {
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
                    if let Some(refracted_ray) = intersection.refracted_ray(ray) {
                        refracted_color = self.recursive_trace(&refracted_ray, depth - 1);
                    }
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

    /// ray trace a pixel
    /// get the camera ray and test the closest intersection with any object
    /// then perform lighting calculations at the closest intersection
    pub fn trace_pixel(&self, u: u32, v: u32) -> Color {
        let ray = self.camera.get_ray_through(u, v);

        self.recursive_trace(&ray, self.camera.get_max_bounces())
    }
}
