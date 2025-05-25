//! scene module

mod camera;
mod light;
mod scene;
mod sphere;

pub use crate::objects::camera::Camera;
pub use crate::objects::light::Light;
pub use crate::objects::scene::Scene;
pub use crate::objects::sphere::{Material, Sphere};
