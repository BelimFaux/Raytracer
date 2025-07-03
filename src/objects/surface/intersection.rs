use crate::{
    math::{Color, Point3, Ray, Vec3, BIAS},
    objects::Light,
};

use super::{Material, Texel};

/// Struct to represent an intersection of a ray and a surface
/// has to live at least as long as the surface, since it borrows its material
pub struct Intersection<'a> {
    pub point: Point3,
    pub t: f32,
    pub normal: Vec3,
    pub texel: Texel,
    pub material: &'a Material,
}

impl Intersection<'_> {
    /// Calculate the color of the intersection point
    pub fn get_color(&self, light: &Light, ray: &Ray) -> Color {
        self.material
            .get_color(&self.point, &self.normal, light, self.texel, ray)
    }

    /// Reflect the given ray at the intersection point
    pub fn reflected_ray(&self, ray: &Ray) -> Ray {
        let dir = Vec3::reflect(ray.dir(), &self.normal);
        Ray::new(self.point + BIAS * dir, dir)
    }

    /// Refract the ray at the intersection point
    /// returns None if total interal refraction happens (no refracted ray has to be sent)
    /// See [here](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html) for derivation
    pub fn refracted_ray(&self, ray: &Ray) -> Option<Ray> {
        let v = ray.dir();
        let mut n = self.normal;
        let mut n_dot_v = n.dot(v);

        // snells law
        let n1_nt = if n_dot_v < 0. {
            // hit from outside
            n_dot_v = -n_dot_v;
            1. / self.material.refraction()
        } else {
            // hit from inside
            n = -n;
            self.material.refraction()
        };

        let discr = 1. - (n1_nt * n1_nt) * (1. - (n_dot_v * n_dot_v));
        // total internal refraction
        if discr < 0. {
            return None;
        }

        let t = n1_nt * (*v + n * n_dot_v) - n * discr.sqrt();

        Some(Ray::new(self.point + BIAS * t, t))
    }

    /// Return the reflectence parameter from the material that was hit
    pub fn get_reflectance(&self) -> f32 {
        self.material.reflectance()
    }

    /// Return the transmittance parameter from the material that was hit
    pub fn get_transmittance(&self) -> f32 {
        self.material.transmittance()
    }
}
