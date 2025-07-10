use std::f32::consts::PI;

use crate::{
    image::Image,
    math::{max, smoothstep, Color, Point3, Ray, Vec3},
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
            Texture::Image(i) => Color::from(i.get_pixel(0, texel.0, texel.1)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShadingModel {
    Phong { ka: f32, kd: f32, ks: f32, exp: u32 },
    CookTorrance { ka: f32, ks: f32, roughness: f32 },
}

impl ShadingModel {
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
    fn cook_torrance_color(
        ctparams: (f32, f32),
        light_color: &Color,
        neg_light: &Vec3,
        vnormal: &Vec3,
        neg_veye: &Vec3,
        frag_color: Color,
    ) -> Color {
        let (ks, alpha) = ctparams;
        let alpha2: f32 = alpha * alpha;
        let f0 = Vec3::new(0.56, 0.57, 0.58);

        let l = -Vec3::normal(neg_light);
        let n = Vec3::normal(vnormal);
        let e = -Vec3::normal(neg_veye);
        let h = Vec3::normal(&(e + l));
        let s = ks;
        let d = 1. - s;

        let ndotl = max(n.dot(&l), 0.);
        let ndote = max(n.dot(&e), 0.);

        // Distribution of the microfacets (GGX)
        let distribution = Self::d_ggx(n, h, alpha2);
        // Geometric shadowing function (microfacets shadow or obstruct light)
        let geo_shadowing = Self::g_ggx(n, h, e, l, alpha2);
        // Fresnel effect
        let fresnel = Self::fresnel(f0, h, e);

        // specular reflection using the cook-torrance model: (DGF) / 4 * (n*l) * (n*v)
        let r_s = (distribution * geo_shadowing * fresnel) / max(4.0 * ndotl * ndote, 0.00001); // dont divide by zero

        let diffuse = frag_color;
        let brdf = d * diffuse + s * r_s;

        *light_color * ndotl * brdf
    }

    /// Calculate the color of the material with a light color
    fn phong_color(
        phparams: (f32, f32, u32),
        light_color: &Color,
        neg_light: &Vec3,
        vnormal: &Vec3,
        neg_veye: &Vec3,
        frag_color: Color,
    ) -> Color {
        let (kd, ks, exp) = phparams;
        let l = Vec3::normal(neg_light);
        let n = -Vec3::normal(vnormal);
        let diffuse = *light_color * frag_color * kd * max(l.dot(&n), 0.0);
        let r = Vec3::reflect(&l, &n);
        let e = -Vec3::normal(neg_veye);
        let specular = *light_color * ks * max(e.dot(&r), 0.0).powf(exp as f32);
        diffuse + specular
    }

    /// Calculate the color of the material with a light color using the specified shading model
    pub fn shading_color(
        &self,
        light_color: &Color,
        neg_light: &Vec3,
        vnormal: &Vec3,
        neg_veye: &Vec3,
        frag_color: Color,
    ) -> Color {
        match self {
            Self::Phong { ka: _, kd, ks, exp } => Self::phong_color(
                (*kd, *ks, *exp),
                light_color,
                neg_light,
                vnormal,
                neg_veye,
                frag_color,
            ),
            Self::CookTorrance {
                ka: _,
                ks,
                roughness,
            } => Self::cook_torrance_color(
                (*ks, *roughness),
                light_color,
                neg_light,
                vnormal,
                neg_veye,
                frag_color,
            ),
        }
    }

    /// get the ambient coefficent of the shading model
    pub fn ambient(&self) -> f32 {
        match self {
            Self::Phong { ka, .. } => *ka,
            Self::CookTorrance { ka, .. } => *ka,
        }
    }
}

/// Struct to represent a Material
#[derive(Clone, Debug)]
pub struct Material {
    reflectance: f32,
    transmittance: f32,
    refraction: f32,
    texture: Texture,
    shading: ShadingModel,
}

impl Material {
    /// Create a new material
    pub fn new(
        texture: Texture,
        reflectance: f32,
        transmittance: f32,
        refraction: f32,
        shading: ShadingModel,
    ) -> Material {
        Material {
            texture,
            reflectance,
            transmittance,
            refraction,
            shading,
        }
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
            Light::Ambient { color } => {
                *color * self.texture.get_color(texel) * self.shading.ambient()
            }
            Light::Parallel { color, direction } => self.shading.shading_color(
                color,
                direction,
                normal,
                ray.dir(),
                self.texture.get_color(texel),
            ),
            Light::Point { color, position } => {
                let dir = *point - *position;
                self.shading.shading_color(
                    color,
                    &dir,
                    normal,
                    ray.dir(),
                    self.texture.get_color(texel),
                )
            }
            Light::Spot {
                color,
                position,
                direction,
                falloff,
            } => {
                let dir = Vec3::normal(&(*point - *position));
                let dot_from_dir = dir.dot(&Vec3::normal(direction));
                let in_light = smoothstep(falloff.1, falloff.0, dot_from_dir);
                if in_light == 0. {
                    Color::zero()
                } else {
                    in_light
                        * self.shading.shading_color(
                            color,
                            &dir,
                            normal,
                            ray.dir(),
                            self.texture.get_color(texel),
                        )
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
