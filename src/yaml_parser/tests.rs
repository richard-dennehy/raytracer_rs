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
        let output = yaml.parse::<CameraDescription>();
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
        let output = yaml.parse::<Light>();
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
        let output = yaml.parse::<Light>();
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
        let define = yaml.parse::<Define>();
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::MaterialDef {
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
        let define = yaml.parse::<Define>();
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::MaterialDef {
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
        let define = yaml.parse::<Define>();
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Transform {
                name: "standard-transform".into(),
                value: vec![
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
                ]
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
        let define = yaml.parse::<Define>();
        assert!(define.is_ok(), define.unwrap_err());
        let define = define.unwrap();

        assert_eq!(
            define,
            Define::Transform {
                name: "large-object".into(),
                value: vec![
                    Transform::Reference("standard-transform".into()),
                    Transform::Scale {
                        x: 3.5,
                        y: 3.5,
                        z: 3.5
                    }
                ]
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
        let object = yaml.parse::<ObjectDescription>();
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
                transform: vec![
                    Transform::RotationX(1.5707963267948966),
                    Transform::Translate {
                        x: 0.0,
                        y: 0.0,
                        z: 500.0
                    }
                ]
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
        let object = yaml.parse::<ObjectDescription>();
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
                transform: vec![Transform::Reference("large-object".into())]
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
        let object = yaml.parse::<ObjectDescription>();
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            object,
            ObjectDescription {
                kind: ObjectKind::Cube,
                material: Left("white-material".into()),
                transform: vec![
                    Transform::Reference("medium-object".into()),
                    Transform::Translate {
                        x: 4.0,
                        y: 0.0,
                        z: 0.0
                    }
                ]
            }
        );
    }

    #[test]
    #[clippy::allow("approx_constant")] // approximation of PI/2 matches the file
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
                defines: vec![
                    Define::MaterialDef {
                        name: "white-material".into(),
                        extends: None,
                        value: MaterialDescription {
                            colour: Some(Colour::WHITE),
                            diffuse: Some(0.7),
                            ambient: Some(0.1),
                            specular: Some(0.0),
                            reflective: Some(0.1),
                            ..Default::default()
                        }
                    },
                    Define::MaterialDef {
                        name: "blue-material".into(),
                        extends: Some("white-material".into()),
                        value: MaterialDescription {
                            colour: Some(Colour::new(0.537, 0.831, 0.914)),
                            ..Default::default()
                        }
                    },
                    Define::MaterialDef {
                        name: "red-material".into(),
                        extends: Some("white-material".into()),
                        value: MaterialDescription {
                            colour: Some(Colour::new(0.941, 0.322, 0.388)),
                            ..Default::default()
                        }
                    },
                    Define::MaterialDef {
                        name: "purple-material".into(),
                        extends: Some("white-material".into()),
                        value: MaterialDescription {
                            colour: Some(Colour::new(0.373, 0.404, 0.550)),
                            ..Default::default()
                        }
                    },
                    Define::Transform {
                        name: "standard-transform".into(),
                        value: vec![
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
                        ]
                    },
                    Define::Transform {
                        name: "large-object".into(),
                        value: vec![
                            Transform::Reference("standard-transform".into()),
                            Transform::Scale {
                                x: 3.5,
                                y: 3.5,
                                z: 3.5
                            }
                        ]
                    },
                    Define::Transform {
                        name: "medium-object".into(),
                        value: vec![
                            Transform::Reference("standard-transform".into()),
                            Transform::Scale {
                                x: 3.0,
                                y: 3.0,
                                z: 3.0
                            }
                        ]
                    },
                    Define::Transform {
                        name: "small-object".into(),
                        value: vec![
                            Transform::Reference("standard-transform".into()),
                            Transform::Scale {
                                x: 2.0,
                                y: 2.0,
                                z: 2.0
                            }
                        ]
                    },
                ],
                objects: vec![
                    ObjectDescription {
                        kind: ObjectKind::Plane,
                        material: Right(MaterialDescription {
                            colour: Some(Colour::WHITE),
                            ambient: Some(1.0),
                            diffuse: Some(0.0),
                            specular: Some(0.0),
                            ..Default::default()
                        }),
                        transform: vec![
                            Transform::RotationX(1.5707963267948966),
                            Transform::Translate {
                                x: 0.0,
                                y: 0.0,
                                z: 500.0
                            }
                        ]
                    },
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
                        transform: vec![Transform::Reference("large-object".into())]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("medium-object".into()),
                            Transform::Translate {
                                x: 4.0,
                                y: 0.0,
                                z: 0.0
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("blue-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 8.5,
                                y: 1.5,
                                z: -0.5
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("red-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 0.0,
                                y: 0.0,
                                z: 4.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("small-object".into()),
                            Transform::Translate {
                                x: 4.0,
                                y: 0.0,
                                z: 4.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("purple-material".into()),
                        transform: vec![
                            Transform::Reference("medium-object".into()),
                            Transform::Translate {
                                x: 7.5,
                                y: 0.5,
                                z: 4.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("medium-object".into()),
                            Transform::Translate {
                                x: -0.25,
                                y: 0.25,
                                z: 8.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("blue-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 4.0,
                                y: 1.0,
                                z: 7.5,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("red-material".into()),
                        transform: vec![
                            Transform::Reference("medium-object".into()),
                            Transform::Translate {
                                x: 10.0,
                                y: 2.0,
                                z: 7.5,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("small-object".into()),
                            Transform::Translate {
                                x: 8.0,
                                y: 2.0,
                                z: 12.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("small-object".into()),
                            Transform::Translate {
                                x: 20.0,
                                y: 1.0,
                                z: 9.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("blue-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: -0.5,
                                y: -5.0,
                                z: 0.25,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("red-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 4.0,
                                y: -4.0,
                                z: 0.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 8.5,
                                y: -4.0,
                                z: 0.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 0.0,
                                y: -4.0,
                                z: 4.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("purple-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: -0.5,
                                y: -4.5,
                                z: 8.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: 0.0,
                                y: -8.0,
                                z: 4.0,
                            }
                        ]
                    },
                    ObjectDescription {
                        kind: ObjectKind::Cube,
                        material: Left("white-material".into()),
                        transform: vec![
                            Transform::Reference("large-object".into()),
                            Transform::Translate {
                                x: -0.5,
                                y: -8.5,
                                z: 8.0,
                            }
                        ]
                    },
                ],
            }
        );
    }
}

mod creating_a_scene {
    use super::*;
    use crate::{Camera, Colour, Material, Matrix4D, Pattern, Vector3D};
    use nonzero_ext::nonzero;

    fn with_camera_description(rest: &str) -> String {
        format!(
            "\
- add: camera
  width: 100
  height: 100
  field-of-view: 0.785
  from: [ -6, 6, -10 ]
  to: [ 6, 0, 6 ]
  up: [ -0.45, 1, 0 ]

{}",
            rest
        )
    }

    #[test]
    fn should_be_able_to_create_a_camera_from_a_valid_file() {
        let input = with_camera_description("");

        let scene = parse(&input);
        assert!(scene.is_ok(), scene.unwrap_err());
        let scene = scene.unwrap();

        let camera = scene.camera();
        assert!(camera.is_ok(), camera.unwrap_err());
        let camera = camera.unwrap();

        assert_eq!(
            camera,
            Camera::new(
                nonzero!(100_u16),
                nonzero!(100_u16),
                0.785,
                Matrix4D::view_transform(
                    Point3D::new(-6.0, 6.0, -10.0),
                    Point3D::new(6.0, 0.0, 6.0),
                    Vector3D::new(-0.45, 1.0, 0.0)
                )
            )
        );
    }

    #[test]
    fn should_be_able_to_create_a_simple_object_with_a_colour_and_no_transforms() {
        let input = with_camera_description(
            "\
- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform: []",
        );

        let scene = parse(&input);
        assert!(scene.is_ok(), scene.unwrap_err());
        let scene = scene.unwrap();

        let objects = scene.objects();
        assert!(objects.is_ok(), objects.unwrap_err());
        let objects = objects.unwrap();

        assert_eq!(objects.len(), 1);
        assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
        assert_eq!(
            objects[0].material,
            Material {
                pattern: Pattern::solid(Colour::new(0.373, 0.404, 0.55)),
                ..Default::default()
            }
        );
        assert_eq!(objects[0].transform(), Matrix4D::identity());
    }
}
