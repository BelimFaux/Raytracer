use crate::math::{Color, Ray};

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
    fn intersection_color(&self, intersect: Intersection, ray: &Ray) -> Color {
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

    /// ray trace a pixel
    /// get the camera ray and test the closest intersection with any object
    /// then perform lighting calculations at the closest intersection
    pub fn trace_pixel(&self, u: u32, v: u32) -> Color {
        let ray = self.camera.get_ray_through(u, v);

        match self.closest_intersection(&ray) {
            Some(intersection) => self.intersection_color(intersection, &ray),
            None => self.background_color,
        }
    }
}
