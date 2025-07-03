use crate::{
    image::Image,
    math::{max, Color, Point3, Ray, Vec3},
    objects::Light,
};

use super::Texel;

/// Texture that defines the color of a material
/// can be either a solid color or a defined by an image
#[derive(Clone, Debug)]
pub enum Texture {
    Color(Color),
    Image(Image),
}

impl Texture {
    /// return the color at a given texel
    pub fn get_color(&self, texel: Texel) -> Color {
        match self {
            Texture::Color(c) => *c,
            Texture::Image(i) => Color::from(i.get_pixel(texel.0, texel.1)),
        }
    }
}

/// Struct to represent a Material
#[derive(Clone, Debug)]
pub struct Material {
    texture: Texture,
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
        texture: Texture,
        reflectance: f32,
        transmittance: f32,
        refraction: f32,
        phong: (f32, f32, f32, u32),
    ) -> Material {
        Material {
            texture,
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
        neg_light: &Vec3,
        vnormal: &Vec3,
        neg_veye: &Vec3,
        texel: Texel,
    ) -> Color {
        let l = Vec3::normal(neg_light);
        let n = -Vec3::normal(vnormal);
        let diffuse = *light_color * self.texture.get_color(texel) * self.kd * max(l.dot(&n), 0.0);
        let r = Vec3::reflect(&l, &n);
        let e = -Vec3::normal(neg_veye);
        let specular = *light_color * self.ks * max(e.dot(&r), 0.0).powf(self.exp as f32);
        diffuse + specular
    }

    /// Calculate the color for the given light source when hitting a point with this material with a ray
    pub fn get_color(
        &self,
        point: &Point3,
        normal: &Vec3,
        light: &Light,
        texel: Texel,
        ray: &Ray,
    ) -> Color {
        match light {
            Light::Ambient { color } => *color * self.texture.get_color(texel) * self.ka,
            Light::Parallel { color, direction } => {
                self.phong(color, direction, normal, ray.dir(), texel)
            }
            Light::Point { color, position } => {
                let dir = *point - *position;
                self.phong(color, &dir, normal, ray.dir(), texel)
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
