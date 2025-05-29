use quick_xml;
use std::fs;

use super::{serial_types::SerialScene, InputError};
use crate::objects::Scene;

pub fn file_to_scene(path: &str) -> Result<Scene, InputError> {
    let content = fs::read_to_string(path).map_err(|err| InputError(err.to_string()))?;

    let scene: SerialScene =
        quick_xml::de::from_str(&content).map_err(|err| InputError(err.to_string()))?;

    Ok(scene.into())
}
