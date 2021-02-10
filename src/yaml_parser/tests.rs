use super::*;

mod unit_tests {
    use super::*;
    use crate::{Colour, Vector3D};

    #[test]
    fn should_parse_camera_description() {
        let input = "\
add: camera
width: 100
height: 100
field-of-view: 0.785
from: [ -6, 6, -10 ]
to: [ 6, 0, 6 ]
up: [ -0.45, 1, 0 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let output = parse_camera(&yaml);
        assert!(output.is_ok(), output.unwrap_err());
        let output = output.unwrap();

        assert_eq!(
            output,
            CameraDescription {
                width: 100,
                height: 100,
                field_of_view: 0.785,
                from: Point3D::new(-6.0, 6.0, -10.0),
                to: Point3D::new(6.0, 0.0, 6.0),
                up: Vector3D::new(-0.45, 1.0, 0.0),
            }
        )
    }

    #[test]
    fn should_parse_a_white_light() {
        let input = "\
add: light
at: [ 50, 100, -50 ]
intensity: [ 1, 1, 1 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let output = parse_light(&yaml);
        assert!(output.is_ok(), output.unwrap_err());
        let output = output.unwrap();

        assert_eq!(
            output,
            Light::point(Colour::WHITE, Point3D::new(50.0, 100.0, -50.0))
        );
    }

    #[test]
    fn should_parse_a_low_intensity_white_light() {
        let input = "\
add: light
at: [ -400, 50, -10 ]
intensity: [ 0.2, 0.2, 0.2 ]
";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let output = parse_light(&yaml);
        assert!(output.is_ok(), output.unwrap_err());
        let output = output.unwrap();

        assert_eq!(
            output,
            Light::point(Colour::greyscale(0.2), Point3D::new(-400.0, 50.0, -10.0))
        );
    }

    #[test]
    fn should_parse_scene_description() {
        let scene = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scene_descriptions/cover.yml"
        ));

        let output = parse(scene);
        assert!(output.is_ok(), output.unwrap_err());
        let output = output.unwrap();
        assert_eq!(
            output,
            SceneDescription {
                camera: CameraDescription {
                    width: 100,
                    height: 100,
                    field_of_view: 0.785,
                    from: Point3D::new(-6.0, 6.0, -10.0),
                    to: Point3D::new(6.0, 0.0, 6.0),
                    up: Vector3D::new(-0.45, 1.0, 0.0),
                },
                lights: vec![
                    Light::Point {
                        colour: Colour::WHITE,
                        position: Point3D::new(50.0, 100.0, -50.0)
                    },
                    Light::Point {
                        colour: Colour::greyscale(0.2),
                        position: Point3D::new(-400.0, 50.0, -10.0)
                    },
                ]
            }
        );
    }
}
