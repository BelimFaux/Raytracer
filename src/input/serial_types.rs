use std::{fs, path::PathBuf};

use crate::{
    image::Image,
    math::{to_radians, Color, Vec3},
    objects::{Camera, Light, Material, Mesh, Scene, Sphere, Surface, Texture},
};
use serde::Deserialize;

use super::{objparser::parse, InputError};

// --- Camera serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct SerialCamera {
    position: Vec3,
    lookat: Vec3,
    up: Vec3,
    horizontal_fov: Fov,
    resolution: Resolution,
    max_bounces: MaxBounces,
}

#[derive(Debug, Deserialize)]
pub(super) struct Fov {
    #[serde(rename = "@angle")]
    angle: u32,
}

#[derive(Debug, Deserialize)]
pub(super) struct Resolution {
    #[serde(rename = "@horizontal")]
    horizontal: u32,
    #[serde(rename = "@vertical")]
    vertical: u32,
}

#[derive(Debug, Deserialize)]
pub(super) struct MaxBounces {
    #[serde(rename = "@n")]
    n: u32,
}

impl From<SerialCamera> for Camera {
    fn from(inp: SerialCamera) -> Camera {
        Camera::new(
            inp.position,
            inp.lookat,
            inp.up,
            to_radians(inp.horizontal_fov.angle),
            inp.resolution.horizontal,
            inp.resolution.vertical,
            inp.max_bounces.n,
        )
    }
}

// --- Material serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct MaterialSolid {
    color: Color,
    phong: Phong,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

#[derive(Debug, Deserialize)]
pub(super) struct MaterialTextured {
    texture: SerialTexture,
    phong: Phong,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

#[derive(Debug, Deserialize)]
pub(super) struct SerialTexture {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct Phong {
    #[serde(rename = "@ka")]
    ka: f32,
    #[serde(rename = "@kd")]
    kd: f32,
    #[serde(rename = "@ks")]
    ks: f32,
    #[serde(rename = "@exponent")]
    exp: u32,
}

impl From<Phong> for (f32, f32, f32, u32) {
    fn from(value: Phong) -> Self {
        (value.ka, value.kd, value.ks, value.exp)
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Reflectance {
    #[serde(rename = "@r")]
    r: f32,
}

#[derive(Debug, Deserialize)]
pub(super) struct Transmittance {
    #[serde(rename = "@t")]
    t: f32,
}

#[derive(Debug, Deserialize)]
pub(super) struct Refraction {
    #[serde(rename = "@iof")]
    iof: f32,
}

impl MaterialTextured {
    fn convert_to_material(self, path: &mut PathBuf) -> Result<Material, InputError> {
        path.set_file_name(self.texture.name);
        let image = Image::load_png(path)?;
        Ok(Material::new(
            Texture::Image(image),
            self.reflectance.r,
            self.transmittance.t,
            self.refraction.iof,
            (self.phong.ka, self.phong.kd, self.phong.ks, self.phong.exp),
        ))
    }
}

impl From<MaterialSolid> for Material {
    fn from(inp: MaterialSolid) -> Material {
        Material::new(
            Texture::Color(inp.color),
            inp.reflectance.r,
            inp.transmittance.t,
            inp.refraction.iof,
            inp.phong.into(),
        )
    }
}

// --- Transform serial types ---

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum Transform {
    Translate {
        #[serde(rename = "@x")]
        x: f32,
        #[serde(rename = "@y")]
        y: f32,
        #[serde(rename = "@z")]
        z: f32,
    },
    Scale {
        #[serde(rename = "@x")]
        x: f32,
        #[serde(rename = "@y")]
        y: f32,
        #[serde(rename = "@z")]
        z: f32,
    },
    RotateX {
        #[serde(rename = "@theta")]
        theta: f32,
    },
    RotateY {
        #[serde(rename = "@theta")]
        theta: f32,
    },
    RotateZ {
        #[serde(rename = "@theta")]
        theta: f32,
    },
}

// --- Surface serial types ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum SerialSurface {
    Sphere {
        #[serde(rename = "@radius")]
        radius: f32,
        position: Vec3,
        material_solid: Option<MaterialSolid>,
        #[allow(unused)]
        material_textured: Option<MaterialTextured>,
        #[allow(unused)]
        transform: Option<TransformList>,
    },
    Mesh {
        #[serde(rename = "@name")]
        name: String,
        material_solid: Option<MaterialSolid>,
        material_textured: Option<MaterialTextured>,
        #[allow(unused)]
        transform: Option<TransformList>,
    },
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct TransformList {
    #[serde(default)]
    #[serde(rename = "$value")]
    transforms: Vec<Transform>,
}

impl SerialSurface {
    /// Converts deserialized surface to a surface
    /// Takes a pathbuf from the path of the xml file, because it will look for obj files in the
    /// same directory
    fn convert_to_surface(self, path: &mut PathBuf) -> Result<Surface, InputError> {
        match self {
            SerialSurface::Sphere {
                radius,
                position,
                material_solid,
                material_textured,
                transform: _,
            } => {
                let material = if let Some(m) = material_solid {
                    m.into()
                } else {
                    material_textured
                        .map(|m| m.convert_to_material(path))
                        .ok_or(InputError::new(format!(
                            "Error while reading file '{}':\n    No material was given.",
                            path.to_str().unwrap_or("<INVALID PATH>")
                        )))??
                };
                Ok(Surface::Sphere(Sphere::new(position, radius, material)))
            }
            SerialSurface::Mesh {
                name,
                material_solid,
                material_textured,
                transform: _,
            } => {
                path.set_file_name(&name);
                let file = fs::read_to_string(&mut *path).map_err(|err| {
                    InputError::new(format!(
                        "Error while reading file '{}':\n    {}",
                        &name, err
                    ))
                })?;
                let material = if let Some(m) = material_solid {
                    m.into()
                } else {
                    material_textured
                        .map(|m| m.convert_to_material(path))
                        .ok_or(InputError::new(format!(
                            "Error while reading file '{}':\n    No material was given.",
                            path.to_str().unwrap_or("<INVALID PATH>")
                        )))??
                };
                let triangles = parse(file).map_err(|err| {
                    InputError::new(format!(
                        "Error while parsing file '{}':\n    {}",
                        &name, err
                    ))
                })?;
                Ok(Surface::Mesh(Box::new(Mesh::new(triangles, material))))
            }
        }
    }
}

// --- Light serial types ---

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct Falloff {
    #[serde(rename = "@alpha1")]
    alpha1: u32,
    #[serde(rename = "@alpha2")]
    alpha2: u32,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum SerialLight {
    AmbientLight {
        color: Color,
    },
    ParallelLight {
        color: Color,
        direction: Vec3,
    },
    PointLight {
        color: Color,
        position: Vec3,
    },
    #[allow(unused)]
    SpotLight {
        color: Color,
        position: Vec3,
        direction: Vec3,
        falloff: Falloff,
    },
}

impl From<SerialLight> for Light {
    fn from(inp: SerialLight) -> Light {
        match inp {
            SerialLight::AmbientLight { color } => Light::Ambient { color },
            SerialLight::ParallelLight { color, direction } => Light::Parallel { color, direction },
            SerialLight::PointLight { color, position } => Light::Point { color, position },
            SerialLight::SpotLight { .. } => unimplemented!("Spotlights are not supported"),
        }
    }
}

// --- Scene serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct SerialScene {
    #[serde(rename = "@output_file")]
    output_file: String,
    background_color: Color,
    camera: SerialCamera,
    lights: LightList,
    surfaces: SurfaceList,
}

#[derive(Debug, Deserialize)]
pub(super) struct LightList {
    #[serde(default)]
    #[serde(rename = "$value")]
    lights: Vec<SerialLight>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SurfaceList {
    #[serde(default)]
    #[serde(rename = "$value")]
    surfaces: Vec<SerialSurface>,
}

impl SerialScene {
    /// Converts deserialized scene to a scene
    /// Takes a pathbuf from the path of the xml file, because it will look for other files in the
    /// same directory
    pub fn convert_to_scene(self, path: &mut PathBuf) -> Result<Scene, InputError> {
        Ok(Scene::new(
            self.output_file,
            self.background_color,
            self.camera.into(),
            self.lights
                .lights
                .into_iter()
                .map(|light| light.into())
                .collect(),
            self.surfaces
                .surfaces
                .into_iter()
                .map(|serial| serial.convert_to_surface(path))
                .collect::<Result<Vec<_>, InputError>>()?,
        ))
    }
}
