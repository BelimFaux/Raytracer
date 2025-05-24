use crate::math::{Point3, Ray, Vector3};

#[derive(Debug)]
pub struct Camera {
    center: Point3,
    height: f32,
    width: f32,
    fov_t: f32,
    aspect: f32,
}

impl Camera {
    pub fn new(
        pos: Point3,
        _lookat: Point3,
        _up: Vector3,
        fov_x: f32,
        horizontal: u32,
        vertical: u32,
    ) -> Camera {
        let aspect = vertical as f32 / horizontal as f32;
        let fov_t = fov_x.tan();
        Camera {
            center: pos,
            height: vertical as f32,
            width: horizontal as f32,
            fov_t,
            aspect,
        }
    }

    pub fn get_ray_through(&self, u: u32, v: u32) -> Ray {
        let x = (((2 * u + 1) as f32 / self.width) - 1.) * self.fov_t;
        let y = (((2 * v + 1) as f32 / self.height) - 1.) * self.fov_t * self.aspect;

        let mut dir = Vector3::new(x, y, -1.);
        dir.normalize();

        Ray::new(self.center, dir)
    }
}
