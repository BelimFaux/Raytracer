use std::{fs, path::PathBuf};

use crate::{
    math::{to_radians, Color, Vector3},
    objects::{Camera, Light, Material, Mesh, Scene, Sphere, Surface},
};
use serde::Deserialize;

use super::{objparser::parse, InputError};

// --- Camera serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct SerialCamera {
    position: Vector3,
    lookat: Vector3,
    up: Vector3,
    horizontal_fov: Fov,
    resolution: Resolution,
    #[allow(unused)]
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
    #[allow(unused)]
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
        )
    }
}

// --- Material serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct MaterialSolid {
    color: Color,
    phong: Phong,
    #[allow(unused)]
    reflectance: Reflectance,
    #[allow(unused)]
    transmittance: Transmittance,
    #[allow(unused)]
    refraction: Refraction,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct MaterialTextured {
    texture: Texture,
    phong: Phong,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct Texture {
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

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct Reflectance {
    #[serde(rename = "@r")]
    r: f32,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct Transmittance {
    #[serde(rename = "@t")]
    t: f32,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(super) struct Refraction {
    #[serde(rename = "@iof")]
    iof: f32,
}

impl From<MaterialSolid> for Material {
    fn from(inp: MaterialSolid) -> Material {
        Material::new(
            inp.color,
            inp.phong.ka,
            inp.phong.kd,
            inp.phong.ks,
            inp.phong.exp,
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
        position: Vector3,
        material_solid: Option<MaterialSolid>,
        #[allow(unused)]
        material_textured: Option<MaterialTextured>,
        #[allow(unused)]
        transform: Option<TransformList>,
    },
    #[allow(unused)]
    Mesh {
        #[serde(rename = "@name")]
        name: String,
        material_solid: Option<MaterialSolid>,
        material_textured: Option<MaterialTextured>,
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
                material_textured: _,
                transform: _,
            } => Ok(Surface::Sphere(Sphere::new(
                position,
                radius,
                material_solid
                    .expect("Only solid materials are implemented")
                    .into(),
            ))),
            SerialSurface::Mesh {
                name,
                material_solid,
                material_textured: _,
                transform: _,
            } => {
                path.set_file_name(&name);
                let file = fs::read_to_string(path).map_err(|err| {
                    InputError(format!("While parsing file '{}:\n    {}", &name, err))
                })?;
                let triangles = parse(file)?;
                Ok(Surface::Mesh(Mesh::new(
                    triangles,
                    material_solid
                        .expect("Only solid materials are implemented")
                        .into(),
                )))
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
        direction: Vector3,
    },
    PointLight {
        color: Color,
        position: Vector3,
    },
    #[allow(unused)]
    SpotLight {
        color: Color,
        position: Vector3,
        direction: Vector3,
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
