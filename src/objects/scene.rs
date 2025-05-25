use crate::math::Color;

use super::{sphere::Intersection, Camera, Light, Sphere};

pub struct Scene {
    background_color: Color,
    camera: Camera,
    lights: Vec<Light>,
    surfaces: Vec<Sphere>,
}

impl Scene {
    pub fn new(
        background_color: Color,
        camera: Camera,
        lights: Vec<Light>,
        surfaces: Vec<Sphere>,
    ) -> Scene {
        Scene {
            background_color,
            camera,
            lights,
            surfaces,
        }
    }

    pub fn trace_pixel(&self, u: u32, v: u32) -> Color {
        let ray = self.camera.get_ray_through(u, v);

        let mut minint: Option<Intersection> = Option::None;
        for sphere in self.surfaces.iter() {
            match sphere.intersection(&ray) {
                Some(intersection) => match &minint {
                    Some(min) => {
                        if intersection.t < min.t {
                            minint = Some(intersection)
                        }
                    }
                    None => minint = Some(intersection),
                },
                None => continue,
            }
        }

        match minint {
            Some(intersection) => {
                let mut col = Color::zero();
                for light in self.lights.iter() {
                    col += intersection.get_color(light)
                }
                col
            }
            None => self.background_color,
        }
    }
}
