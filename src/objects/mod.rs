//! objects module
//! contains objects that lie inside of the scene

mod camera;
mod light;
mod scene;
mod surface;

pub use crate::objects::camera::Camera;
pub use crate::objects::light::Light;
pub use crate::objects::scene::Scene;
pub use crate::objects::surface::{Material, Sphere};
