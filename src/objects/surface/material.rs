use crate::{
    math::{max, Color, Point3, Ray, Vector3},
    objects::Light,
};

/// Struct to represent a Material
#[derive(Clone, Debug)]
pub struct Material {
    color: Color,
    reflectance: f32,
    transmittance: f32,
    refraction: f32,
    ka: f32,
    kd: f32,
    ks: f32,
    exp: u32,
}

impl Material {
    /// Create a new material
    pub fn new(
        color: Color,
        reflectance: f32,
        transmittance: f32,
        refraction: f32,
        phong: (f32, f32, f32, u32),
    ) -> Material {
        Material {
            color,
            reflectance,
            transmittance,
            refraction,
            ka: phong.0,
            kd: phong.1,
            ks: phong.2,
            exp: phong.3,
        }
    }

    /// Calculate the color of the material with a light color
    fn phong(
        &self,
        light_color: &Color,
        neg_light: &Vector3,
        vnormal: &Vector3,
        neg_veye: &Vector3,
    ) -> Color {
        let l = Vector3::normal(neg_light);
        let n = -Vector3::normal(vnormal);
        let diffuse = self.color * self.kd * max(l.dot(&n), 0.0);
        let r = Vector3::reflect(&l, &n);
        let e = -Vector3::normal(neg_veye);
        let specular = *light_color * self.ks * max(e.dot(&r), 0.0).powf(self.exp as f32);
        diffuse + specular
    }

    /// Calculate the color for the given light source when hitting a point with this material with a ray
    pub fn get_color(&self, point: &Point3, normal: &Vector3, light: &Light, ray: &Ray) -> Color {
        match light {
            Light::Ambient { .. } => self.color * self.ka,
            Light::Parallel { color, direction } => self.phong(color, direction, normal, ray.dir()),
            Light::Point { color, position } => {
                let dir = *point - *position;
                self.phong(color, &dir, normal, ray.dir())
            }
        }
    }

    /// Getter for the reflectance
    pub fn reflectance(&self) -> f32 {
        self.reflectance
    }

    /// Getter for the transmittance
    pub fn transmittance(&self) -> f32 {
        self.transmittance
    }

    /// Getter for the refraction
    pub fn refraction(&self) -> f32 {
        self.refraction
    }
}
