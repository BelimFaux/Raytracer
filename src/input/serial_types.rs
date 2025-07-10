use std::{fs, path::PathBuf};

use crate::{
    image::Image,
    math::{to_radians, Color, Mat4, Point3, Quat, Vec3},
    objects::{Camera, Light, Material, Scene, ShadingModel, Surface, Texture},
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
    depth_of_field: Option<DepthOfField>,
    max_bounces: MaxBounces,
}

#[derive(Debug, Deserialize)]
pub(super) struct DepthOfField {
    #[serde(rename = "@focal_length")]
    focal_length: f32,
    #[serde(rename = "@aperture")]
    aperture: f32,
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
        let mut c = Camera::new(
            inp.position,
            inp.lookat,
            inp.up,
            to_radians(inp.horizontal_fov.angle as f32),
            inp.resolution.horizontal,
            inp.resolution.vertical,
            inp.max_bounces.n,
        );
        if let Some(dof) = inp.depth_of_field {
            c.add_dof(dof.focal_length, dof.aperture);
        }
        c
    }
}

// --- Material serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct MaterialSolid {
    color: Color,
    #[serde(rename = "$value")]
    shading: SerialShadingModel,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

#[derive(Debug, Deserialize)]
pub(super) struct MaterialTextured {
    texture: SerialTexture,
    #[serde(rename = "$value")]
    shading: SerialShadingModel,
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
#[serde(rename_all = "snake_case")]
pub(super) enum SerialShadingModel {
    CookTorrance(CookTorrance),
    Phong(Phong),
}

#[derive(Debug, Deserialize)]
pub(super) struct CookTorrance {
    #[serde(rename = "@ka")]
    ka: f32,
    #[serde(rename = "@ks")]
    ks: f32,
    #[serde(rename = "@roughness")]
    roughness: f32,
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

impl From<SerialShadingModel> for ShadingModel {
    fn from(value: SerialShadingModel) -> Self {
        match value {
            SerialShadingModel::Phong(p) => ShadingModel::Phong {
                ka: p.ka,
                kd: p.kd,
                ks: p.ks,
                exp: p.exp,
            },
            SerialShadingModel::CookTorrance(c) => ShadingModel::CookTorrance {
                ka: c.ka,
                ks: c.ks,
                roughness: c.roughness,
            },
        }
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
            self.shading.into(),
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
            inp.shading.into(),
        )
    }
}

// --- Transform serial types ---

#[derive(Debug, Deserialize, Clone)]
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

impl From<Transform> for Mat4 {
    /// converts transform to the inverse of the transformation matrix
    fn from(value: Transform) -> Self {
        match value {
            Transform::Translate { x, y, z } => Mat4::from_translation(Vec3::new(-x, -y, -z)),
            Transform::RotateX { theta } => Mat4::from_x_rotation(to_radians(-theta)),
            Transform::RotateY { theta } => Mat4::from_y_rotation(to_radians(-theta)),
            Transform::RotateZ { theta } => Mat4::from_z_rotation(to_radians(-theta)),
            Transform::Scale { x, y, z } => Mat4::from_scaling(Vec3::new(1. / x, 1. / y, 1. / z)),
        }
    }
}

// --- Surface serial types ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum SerialSurface {
    Sphere {
        #[serde(rename = "@radius")]
        radius: f32,
        #[serde(rename = "@endradius")]
        endradius: Option<f32>,
        position: Vec3,
        endposition: Option<Vec3>,
        material_solid: Option<MaterialSolid>,
        material_textured: Option<MaterialTextured>,
        transform: Option<TransformList>,
    },
    Mesh {
        #[serde(rename = "@name")]
        name: String,
        material_solid: Option<MaterialSolid>,
        material_textured: Option<MaterialTextured>,
        transform: Option<TransformList>,
    },
    JuliaSet {
        #[serde(rename = "@max_iteration")]
        max_iterations: u32,
        #[serde(rename = "@epsilon")]
        epsilon: f32,
        position: Point3,
        constant: SerialQuat,
        endconstant: Option<SerialQuat>,
        material_solid: MaterialSolid,
        transform: Option<TransformList>,
    },
}

#[derive(Debug, Deserialize)]
pub(super) struct SerialQuat {
    #[serde(rename = "@x")]
    x: f32,
    #[serde(rename = "@y")]
    y: f32,
    #[serde(rename = "@z")]
    z: f32,
    #[serde(rename = "@w")]
    w: f32,
}

#[derive(Debug, Deserialize)]
pub(super) struct TransformList {
    #[serde(default)]
    #[serde(rename = "$value")]
    transforms: Vec<Transform>,
}

impl From<TransformList> for Mat4 {
    /// Calculate the final inverse transformation matrix
    fn from(value: TransformList) -> Self {
        value
            .transforms
            .iter()
            .fold(Mat4::identity(), |acc, curr| &curr.clone().into() * &acc)
    }
}

impl SerialSurface {
    /// Converts deserialized surface to a surface
    /// Takes a pathbuf from the path of the xml file, because it will look for obj files in the
    /// same directory
    fn convert_to_surface(self, path: &mut PathBuf) -> Result<Surface, InputError> {
        match self {
            SerialSurface::Sphere {
                radius,
                endradius,
                position,
                endposition,
                material_solid,
                material_textured,
                transform,
            } => {
                let material = if let Some(m) = material_solid {
                    m.into()
                } else {
                    material_textured
                        .map(|m| m.convert_to_material(path))
                        .ok_or(InputError::new(
                            format!(
                                "Error while reading file '{}':",
                                path.to_str().unwrap_or("<INVALID PATH>")
                            ),
                            "No material was given.".to_string(),
                        ))??
                };
                let mut sphere = Surface::sphere(position, radius, material);
                if let Some(t) = transform {
                    let inv_transform = t.into();
                    let normal_transform = Mat4::transpose(&inv_transform);
                    sphere.set_transform(inv_transform, normal_transform);
                }
                if endradius.is_some() || endposition.is_some() {
                    let ec = endposition.unwrap_or(position);
                    let er = endradius.unwrap_or(radius);
                    sphere.set_sphere_end((ec, er));
                }
                Ok(sphere)
            }
            SerialSurface::Mesh {
                name,
                material_solid,
                material_textured,
                transform,
            } => {
                path.set_file_name(&name);
                let file = fs::read_to_string(&mut *path).map_err(|err| {
                    InputError::new(
                        format!("Error while reading file '{}'", &name),
                        err.to_string(),
                    )
                })?;
                let material = if let Some(m) = material_solid {
                    m.into()
                } else {
                    material_textured
                        .map(|m| m.convert_to_material(path))
                        .ok_or(InputError::new(
                            format!(
                                "Error while reading file '{}':",
                                path.to_str().unwrap_or("<INVALID PATH>")
                            ),
                            "No material was given.".to_string(),
                        ))??
                };
                let triangles = parse(file).map_err(|err| {
                    InputError::new(format!("Error while parsing file '{}'", &name), err.msg)
                })?;
                let mut surface = Surface::mesh(triangles, material);
                if let Some(t) = transform {
                    let inv_transform = t.into();
                    // normal matrix is the inverse transpose
                    let normal_transform = Mat4::transpose(&inv_transform);
                    surface.set_transform(inv_transform, normal_transform);
                }
                Ok(surface)
            }
            Self::JuliaSet {
                position,
                max_iterations,
                epsilon,
                constant,
                endconstant,
                material_solid,
                transform,
            } => {
                let c = Quat::new(constant.x, constant.y, constant.z, constant.w);
                let mut julia =
                    Surface::julia_set(position, c, max_iterations, epsilon, material_solid.into());
                if let Some(t) = transform {
                    let inv_transform = t.into();
                    // normal matrix is the inverse transpose
                    let normal_transform = Mat4::transpose(&inv_transform);
                    julia.set_transform(inv_transform, normal_transform);
                }
                if let Some(ec) = endconstant {
                    let ec = Quat::new(ec.x, ec.y, ec.z, ec.w);
                    julia.set_julia_end(ec);
                }
                Ok(julia)
            }
        }
    }
}

// --- Light serial types ---

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
            SerialLight::SpotLight {
                color,
                position,
                direction,
                falloff,
            } => Light::Spot {
                color,
                position,
                direction,
                falloff: (
                    to_radians(falloff.alpha1 as f32).cos(),
                    to_radians(falloff.alpha2 as f32).cos(),
                ),
            },
        }
    }
}

// --- Scene serial types ---

#[derive(Debug, Deserialize)]
pub(super) struct SerialScene {
    #[serde(rename = "@output_file")]
    output_file: String,
    background_color: Color,
    super_sampling: Option<SuperSampling>,
    animated: Option<Animated>,
    camera: SerialCamera,
    lights: LightList,
    surfaces: SurfaceList,
}

#[derive(Debug, Deserialize)]
pub(super) struct Animated {
    #[serde(rename = "@frames")]
    frames: usize,
    #[serde(rename = "@fps")]
    fps: u16,
}

#[derive(Debug, Deserialize)]
pub(super) struct SuperSampling {
    #[serde(rename = "@samples")]
    samples: u32,
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
        let mut s = Scene::new(
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
        );
        if let Some(ssaa) = self.super_sampling {
            s.add_samples(ssaa.samples);
        }
        if let Some(anim) = self.animated {
            s.set_animation(anim.frames, anim.fps);
        }

        Ok(s)
    }
}
