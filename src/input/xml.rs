use quick_xml;
use std::fs;

use super::serial_types::SerialScene;
use crate::objects::Scene;

pub fn file_to_scene(path: &str) -> Scene {
    let content = fs::read_to_string(path).expect("Error while accessing file");
    let scene: SerialScene = quick_xml::de::from_str(&content).expect("Error while parsing");

    scene.into()
}
