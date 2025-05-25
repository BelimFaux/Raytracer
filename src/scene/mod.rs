//! scene module

mod camera;
mod light;
mod sphere;

pub use crate::scene::camera::Camera;
pub use crate::scene::light::Light;
pub use crate::scene::sphere::{Material, Sphere};
