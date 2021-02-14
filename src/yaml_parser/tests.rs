use super::*;

mod basic_parsing {
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
    fn should_parse_basic_material_define() {
        let input = "\
define: white-material
value:
  color: [ 1, 1, 1 ]
  diffuse: 0.7
  ambient: 0.1
  specular: 0.0
  reflective: 0.1";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let define = parse_define(yaml, "white-material");
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Material {
                name: "white-material".into(),
                extends: None,
                value: MaterialDescription {
                    colour: Some(Colour::WHITE),
                    diffuse: Some(0.7),
                    ambient: Some(0.1),
                    specular: Some(0.0),
                    shininess: None,
                    reflective: Some(0.1),
                    transparency: None,
                    refractive: None
                }
            }
        );
    }

    #[test]
    fn should_parse_a_material_extending_another_material() {
        let input = "\
define: blue-material
extend: white-material
value:
  color: [ 0.537, 0.831, 0.914 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let define = parse_define(yaml, "blue-material");
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Material {
                name: "blue-material".into(),
                extends: Some("white-material".into()),
                value: MaterialDescription {
                    colour: Some(Colour::new(0.537, 0.831, 0.914)),
                    ..Default::default()
                }
            }
        )
    }

    #[test]
    fn should_parse_a_transform_define() {
        let input = "\
define: standard-transform
value:
  - [ translate, 1, -1, 1 ]
  - [ scale, 0.5, 0.5, 0.5 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let define = parse_define(&yaml, "standard-transform");
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Transform {
                name: "standard-transform".into(),
                value: TransformDescription(vec![
                    Transform::Translate {
                        x: 1.0,
                        y: -1.0,
                        z: 1.0
                    },
                    Transform::Scale {
                        x: 0.5,
                        y: 0.5,
                        z: 0.5
                    }
                ])
            }
        );
    }

    #[test]
    fn should_parse_a_transform_referencing_another_transform() {
        let input = "\
define: large-object
value:
  - standard-transform
  - [ scale, 3.5, 3.5, 3.5 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let define = parse_define(&yaml, "large-object");
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Transform {
                name: "large-object".into(),
                value: TransformDescription(vec![
                    Transform::Reference("standard-transform".into()),
                    Transform::Scale {
                        x: 3.5,
                        y: 3.5,
                        z: 3.5
                    }
                ])
            }
        );
    }

    #[test]
    #[clippy::allow("approx_constant")] // it's no good comparing to PI/2 constant because it won't match
    fn should_parse_simple_plane_description() {
        let input = "\
add: plane
material:
  color: [ 1, 1, 1 ]
  ambient: 1
  diffuse: 0
  specular: 0
transform:
  - [ rotate-x, 1.5707963267948966 ] # pi/2
  - [ translate, 0, 0, 500 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let object = parse_object(&yaml, "plane");
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            object,
            ObjectDescription {
                kind: ObjectKind::Plane,
                material: Right(MaterialDescription {
                    colour: Some(Colour::WHITE),
                    ambient: Some(1.0),
                    diffuse: Some(0.0),
                    specular: Some(0.0),
                    ..Default::default()
                }),
                transform: TransformDescription(vec![
                    Transform::RotationX(1.5707963267948966),
                    Transform::Translate {
                        x: 0.0,
                        y: 0.0,
                        z: 500.0
                    }
                ])
            }
        );
    }

    #[test]
    fn should_parse_simple_sphere_description() {
        let input = "\
add: sphere
material:
  color: [ 0.373, 0.404, 0.550 ]
  diffuse: 0.2
  ambient: 0.0
  specular: 1.0
  shininess: 200
  reflective: 0.7
  transparency: 0.7
  refractive-index: 1.5
transform:
  - large-object";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let object = parse_object(&yaml, "sphere");
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            object,
            ObjectDescription {
                kind: ObjectKind::Sphere,
                material: Right(MaterialDescription {
                    colour: Some(Colour::new(0.373, 0.404, 0.550)),
                    diffuse: Some(0.2),
                    ambient: Some(0.0),
                    specular: Some(1.0),
                    shininess: Some(200.0),
                    reflective: Some(0.7),
                    transparency: Some(0.7),
                    refractive: Some(1.5),
                }),
                transform: TransformDescription(vec![Transform::Reference("large-object".into())])
            }
        );
    }

    #[test]
    fn should_parse_a_cube_referencing_a_material_define() {
        let input = "\
add: cube
material: white-material
transform:
  - medium-object
  - [ translate, 4, 0, 0 ]";

        let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
        let object = parse_object(&yaml, "cube");
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            object,
            ObjectDescription {
                kind: ObjectKind::Cube,
                material: Left("white-material".into()),
                transform: TransformDescription(vec![
                    Transform::Reference("medium-object".into()),
                    Transform::Translate {
                        x: 4.0,
                        y: 0.0,
                        z: 0.0
                    }
                ])
            }
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
                ],
                defines: vec![],
                objects: vec![],
            }
        );
    }
}
