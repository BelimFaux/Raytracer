use std::f32::consts::PI;

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
    cook_torrance: bool,
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
            cook_torrance: false,
        }
    }

    /// use the cook torrance model, rather than the phong model for light calculations
    pub fn use_cook_torrance(&mut self) {
        self.cook_torrance = true
    }

    /// part of ggx geometric shadowing
    fn g1(x: Vec3, h: Vec3, n: Vec3, alpha2: f32) -> f32 {
        let xdotn = max(x.dot(&n), 0.0);

        let chi = if x.dot(&h) / x.dot(&n) > 0.0 {
            1.0
        } else {
            0.0
        };

        let tan2x = (1.0 - (xdotn * xdotn)) / (xdotn * xdotn);
        chi * 2.0 / max(1.0 + (1.0 + alpha2 * tan2x).sqrt(), 0.00001)
    }

    /// GGX geometric shadowing function
    fn g_ggx(n: Vec3, h: Vec3, e: Vec3, l: Vec3, alpha2: f32) -> f32 {
        Self::g1(e, h, n, alpha2) * Self::g1(l, h, n, alpha2)
    }

    /// GGX normal distribution function
    fn d_ggx(n: Vec3, h: Vec3, alpha2: f32) -> f32 {
        let hdotn = max(h.dot(&n), 0.0);

        let chi = if h.dot(&n) > 0.0 { 1.0 } else { 0.0 };
        let mut denom = hdotn * hdotn * (alpha2 - 1.0) + 1.0;
        denom = max(PI * denom * denom, 0.00001);
        chi * alpha2 / denom
    }

    /// fresnel effect using Schlick's approximation
    fn fresnel(f0: Vec3, h: Vec3, v: Vec3) -> Vec3 {
        let vdoth = max(v.dot(&h), 0.0);
        f0 + (Vec3::new(1., 1., 1.) - f0) * (1.0 - vdoth).powf(5.0)
    }

    /// Calculates the color according to the [cook-torrance model](https://graphicscompendium.com/references/cook-torrance)
    fn cook_torrance(
        &self,
        light_color: &Color,
        neg_light: &Vec3,
        vnormal: &Vec3,
        neg_veye: &Vec3,
        texel: Texel,
    ) -> Color {
        const ALPHA: f32 = 0.25;
        const ALPHA2: f32 = ALPHA * ALPHA;
        let f0 = Vec3::new(0.56, 0.57, 0.58);

        let l = -Vec3::normal(neg_light);
        let n = Vec3::normal(vnormal);
        let e = -Vec3::normal(neg_veye);
        let h = Vec3::normal(&(e + l));
        let d = self.kd;
        let s = 1. - d;

        let ndotl = max(n.dot(&l), 0.);
        let ndote = max(n.dot(&e), 0.);

        // Distribution of the microfacets (GGX)
        let distribution = Self::d_ggx(n, h, ALPHA2);
        // Geometric shadowing function (microfacets shadow or obstruct light)
        let geo_shadowing = Self::g_ggx(n, h, e, l, ALPHA2);
        // Fresnel effect
        let fresnel = Self::fresnel(f0, h, e);

        // specular reflection using the cook-torrance model: (DGF) / 4 * (n*l) * (n*v)
        let r_s = (distribution * geo_shadowing * fresnel) / max(4.0 * ndotl * ndote, 0.00001); // dont divide by zero

        let diffuse = self.texture.get_color(texel);
        let brdf = d * diffuse + s * r_s;

        *light_color * ndotl * brdf
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
                if self.cook_torrance {
                    self.cook_torrance(color, direction, normal, ray.dir(), texel)
                } else {
                    self.phong(color, direction, normal, ray.dir(), texel)
                }
            }
            Light::Point { color, position } => {
                let dir = *point - *position;
                if self.cook_torrance {
                    self.cook_torrance(color, &dir, normal, ray.dir(), texel)
                } else {
                    self.phong(color, &dir, normal, ray.dir(), texel)
                }
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
