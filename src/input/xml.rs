use quick_xml;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use super::{serial_types::SerialScene, InputError};
use crate::objects::Scene;

/// convert any error to a specific input error
fn err_to_input_err<E>(err: E, path: &Path) -> InputError
where
    E: Error,
{
    InputError::new(format!(
        "Error while parsing xml file {}:\n    {err}",
        path.to_str().unwrap_or("<INVALID PATH>")
    ))
}

/// Read in an xml fie from the specified path and parse to a scene object
/// The xml file should have the correct format as specified [here](https://teaching.vda.univie.ac.at/graphics/25s/Labs/Lab3/lab2_file_specification.html)
pub fn file_to_scene(path: &str) -> Result<Scene, InputError> {
    let mut path = PathBuf::from(path);
    let content = fs::read_to_string(&path).map_err(|err| err_to_input_err(err, &path))?;

    let scene: SerialScene =
        quick_xml::de::from_str(&content).map_err(|err| err_to_input_err(err, &path))?;

    scene.convert_to_scene(&mut path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_example_no_panic() {
        let xml = r#"
        <?xml version="1.0" standalone="no" ?>
        <!DOCTYPE scene SYSTEM "scene.dtd">

        <scene output_file="myImage.png">
            <background_color r="1.0" g="0.0" b="0.0"/>
            <camera>
                <position x="1.0" y="-2.0E-10" z="-3"/>
                <lookat x="1" y="2" z="3"/>
                <up x="1" y="2" z="3"/>
                <horizontal_fov angle="90"/>
                <resolution horizontal="1920" vertical="1080"/>
                <max_bounces n="100"/>
            </camera>
            <lights>
                <ambient_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                </ambient_light>
                <point_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <position x="1" y="2" z="3"/>
                </point_light>
                <parallel_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <direction x="1" y="2" z="3"/>
                </parallel_light>
                <spot_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <position x="1" y="2" z="3"/>
                    <direction x="1" y="2" z="3"/>
                    <falloff alpha1="1" alpha2="3"/>
                </spot_light>
            </lights>
            <surfaces>
                <sphere radius="123">
                    <position x="1" y="2" z="3"/>
                    <material_solid>
                        <color r="0.1" g="0.2" b="0.3"/>
                        <phong ka="1.0" kd="1.0" ks="1.0" exponent="1"/>
                        <reflectance r="1.0"/>
                        <transmittance t="1.0"/>
                        <refraction iof="1.0"/>
                    </material_solid>
                    <transform>
                        <translate x="1" y="1" z="1"/>
                        <scale x="1" y="1" z="1"/>
                        <rotateX theta="1"/>
                        <rotateY theta="1"/>
                        <rotateZ theta="1"/>
                    </transform>
                </sphere>
                <mesh name="duck.dae">
                    <material_textured>
                        <texture name=""/>
                        <phong ka="1.0" kd="1.0" ks="1.0" exponent="1"/>
                        <reflectance r="1.0"/>
                        <transmittance t="1.0"/>
                        <refraction iof="1.0"/>
                    </material_textured>
                    <transform>
                        <translate x="1" y="1" z="1"/>
                        <scale x="1" y="1" z="1"/>
                        <rotateX theta="1"/>
                        <rotateY theta="1"/>
                        <rotateZ theta="1"/>
                        <translate x="1" y="1" z="1"/>
                        <scale x="1" y="1" z="1"/>
                    </transform>
                </mesh>
            </surfaces>
        </scene>
        "#;

        let _: SerialScene = quick_xml::de::from_str(xml).unwrap();
    }

    #[test]
    fn parse_lab3a_example_correct_fields() {
        let xml = r#"
        <?xml version="1.0" standalone="no" ?>
        <!DOCTYPE scene SYSTEM "scene.dtd">

        <scene output_file="myImage.png">
            <background_color r="1.0" g="0.0" b="0.0"/>
            <camera>
                <position x="1.0" y="-2.0E-10" z="-3"/>
                <lookat x="1" y="2" z="3"/>
                <up x="1" y="2" z="3"/>
                <horizontal_fov angle="90"/>
                <resolution horizontal="1920" vertical="1080"/>
                <max_bounces n="100"/>
            </camera>
            <lights>
                <ambient_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                </ambient_light>
                <point_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <position x="1" y="2" z="3"/>
                </point_light>
                <parallel_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <direction x="1" y="2" z="3"/>
                </parallel_light>
            </lights>
            <surfaces>
                <sphere radius="123">
                    <position x="1" y="2" z="3"/>
                    <material_solid>
                        <color r="0.1" g="0.2" b="0.3"/>
                        <phong ka="1.0" kd="1.0" ks="1.0" exponent="1"/>
                        <reflectance r="1.0"/>
                        <transmittance t="1.0"/>
                        <refraction iof="1.0"/>
                    </material_solid>
                </sphere>
            </surfaces>
        </scene>
        "#;

        let serial_scene: SerialScene = quick_xml::de::from_str(xml).unwrap();
        let scene: Scene = serial_scene.convert_to_scene(&mut PathBuf::new()).unwrap();

        assert_eq!(scene.get_output(), "myImage.png");
        assert_eq!(scene.get_dimensions(), (1920, 1080));
    }

    #[test]
    fn extension_parameters_no_panic() {
        let xml = r#"
        <?xml version="1.0" standalone="no" ?>
        <!DOCTYPE scene SYSTEM "scene.dtd">

        <scene output_file="myImage.png">
            <background_color r="1.0" g="0.0" b="0.0"/>
            <super_sampling samples="16" />
            <camera>
                <position x="1.0" y="-2.0E-10" z="-3"/>
                <lookat x="1" y="2" z="3"/>
                <up x="1" y="2" z="3"/>
                <horizontal_fov angle="90"/>
                <depth_of_field focal_length="2.5" aperture="0.2" />
                <resolution horizontal="1920" vertical="1080"/>
                <max_bounces n="100"/>
            </camera>
            <lights>
                <ambient_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                </ambient_light>
                <point_light>
                    <color r="0.1" g="0.2" b="0.3"/>
                    <position x="1" y="2" z="3"/>
                </point_light>
            </lights>
            <surfaces>
                <sphere radius="123">
                    <position x="1" y="2" z="3"/>
                    <material_solid>
                        <color r="0.1" g="0.2" b="0.3"/>
                        <cook_torrance ka="1.0" ks="1.0" roughness="0.2"/>
                        <reflectance r="1.0"/>
                        <transmittance t="1.0"/>
                        <refraction iof="1.0"/>
                    </material_solid>
                </sphere>
            </surfaces>
        </scene>
        "#;

        let serial_scene: SerialScene = quick_xml::de::from_str(xml).unwrap();

        assert!(serial_scene.convert_to_scene(&mut PathBuf::new()).is_ok());
    }
}
