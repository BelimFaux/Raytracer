use crate::math::{Mat4, Point3, Ray, Vec3};

/// Struct to represent a camera in 3D space
#[derive(Debug)]
pub struct Camera {
    height: f32,
    width: f32,
    fov_t: f32,
    aspect: f32,
    max_bounces: u32,
    transform: Mat4,
    dof: Option<(f32, f32)>,
}

impl Camera {
    /// Create a new camera
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn new(
        pos: Point3,
        lookat: Point3,
        up: Vec3,
        fov_x: f32,
        horizontal: u32,
        vertical: u32,
        max_bounces: u32,
    ) -> Camera {
        let aspect = vertical as f32 / horizontal as f32;
        let fov_t = fov_x.tan();
        Camera {
            height: vertical as f32,
            width: horizontal as f32,
            fov_t,
            aspect,
            max_bounces,
            transform: Mat4::look_at(pos, lookat, up),
            dof: None,
        }
    }

    /// Add depth of field parameters to the camera
    pub fn add_dof(&mut self, focal_distance: f32, aperture: f32) {
        self.dof = Some((focal_distance, aperture));
    }

    /// Return the image dimensions of the camera
    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    /// Return the maximum bounces for the camera
    #[must_use]
    pub fn get_max_bounces(&self) -> u32 {
        self.max_bounces
    }

    fn compute_camera_ray(&self, u: f32, v: f32) -> Ray {
        let x = (((2. * u + 1.) / self.width) - 1.) * self.fov_t;
        let y = (((2. * v + 1.) / self.height) - 1.) * self.fov_t * self.aspect;

        let pcamera = Vec3::new(x, y, -1.);
        let orig = Point3::zero();

        // offset ray if dof is set
        if let Some((focal_distance, aperture)) = self.dof {
            let focal_point = focal_distance * pcamera;
            let orig = orig
                + Vec3::new(
                    rand::random_range(-aperture..aperture),
                    rand::random_range(-aperture..aperture),
                    0.,
                );
            let dir = focal_point - orig;

            Ray::new(orig, dir).transform(&self.transform).normal()
        } else {
            Ray::new(orig, pcamera).transform(&self.transform).normal()
        }
    }

    /// Construct a camera ray through pixel `(u, v)`
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn get_ray_through(&self, u: u32, v: u32) -> Ray {
        self.compute_camera_ray(u as f32, v as f32)
    }

    /// Return a randomly sampled ray through the pixel `(u, v)`
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn get_sample_ray_through(&self, u: u32, v: u32) -> Ray {
        let u = u as f32 + rand::random_range(-0.5..0.5);
        let v = v as f32 + rand::random_range(-0.5..0.5);
        self.compute_camera_ray(u, v)
    }
}
