use super::*;
use crate::{Colour, Light, Point3D, Vector3D};
use either::Either::{Left, Right};

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
    assert!(output.is_ok(), "{}", output.unwrap_err());
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
    assert!(output.is_ok(), "{}", output.unwrap_err());
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
    assert!(output.is_ok(), "{}", output.unwrap_err());
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
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::MaterialDef {
            name: "white-material".into(),
            extends: None,
            value: MaterialDescription {
                pattern: Some(Left(Colour::WHITE)),
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
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::MaterialDef {
            name: "blue-material".into(),
            extends: Some("white-material".into()),
            value: MaterialDescription {
                pattern: Some(Left(Colour::new(0.537, 0.831, 0.914))),
                ..Default::default()
            }
        }
    )
}

#[test]
#[allow(clippy::approx_constant)]
fn should_parse_a_material_define_using_a_stripes_pattern() {
    let input = "\
define: wall-material
value:
  pattern:
    type: stripes
    colors:
      - [0.45, 0.45, 0.45]
      - [0.55, 0.55, 0.55]
    transform:
      - [ scale, 0.25, 0.25, 0.25 ]
      - [ rotate-y, 1.5708 ]
  ambient: 0
  diffuse: 0.4
  specular: 0
  reflective: 0.3";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let define = yaml.parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::MaterialDef {
            name: "wall-material".into(),
            extends: None,
            value: MaterialDescription {
                pattern: Some(Right(PatternDescription {
                    pattern_type: PatternType::Stripes,
                    colours: (Colour::greyscale(0.45), Colour::greyscale(0.55)),
                    transforms: Some(vec![
                        Transformation::Scale {
                            x: 0.25,
                            y: 0.25,
                            z: 0.25
                        },
                        Transformation::RotationY(1.5708)
                    ])
                })),
                ambient: Some(0.0),
                diffuse: Some(0.4),
                specular: Some(0.0),
                reflective: Some(0.3),
                ..Default::default()
            }
        }
    )
}

#[test]
fn should_parse_a_material_define_using_a_checker_pattern() {
    let input = "\
define: checkered-material
value:
  pattern:
    type: checkers
    colors:
      - [0.35, 0.35, 0.35]
      - [0.65, 0.65, 0.65]
  specular: 0
  reflective: 0.4";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let define = yaml.parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::MaterialDef {
            name: "checkered-material".into(),
            extends: None,
            value: MaterialDescription {
                pattern: Some(Right(PatternDescription {
                    pattern_type: PatternType::Checker,
                    colours: (Colour::greyscale(0.35), Colour::greyscale(0.65)),
                    transforms: None
                })),
                specular: Some(0.0),
                reflective: Some(0.4),
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
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Transform {
            name: "standard-transform".into(),
            value: vec![
                Right(Transformation::Translate {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0
                }),
                Right(Transformation::Scale {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                })
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
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Transform {
            name: "large-object".into(),
            value: vec![
                Left("standard-transform".into()),
                Right(Transformation::Scale {
                    x: 3.5,
                    y: 3.5,
                    z: 3.5
                })
            ]
        }
    );
}

#[test]
#[allow(clippy::approx_constant)]
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
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Plane,
            material: MaterialSource::Inline(MaterialDescription {
                pattern: Some(Left(Colour::WHITE)),
                ambient: Some(1.0),
                diffuse: Some(0.0),
                specular: Some(0.0),
                ..Default::default()
            }),
            transform: vec![
                Right(Transformation::RotationX(1.5707963267948966)),
                Right(Transformation::Translate {
                    x: 0.0,
                    y: 0.0,
                    z: 500.0
                })
            ],
            casts_shadow: true,
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
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Sphere,
            material: MaterialSource::Inline(MaterialDescription {
                pattern: Some(Left(Colour::new(0.373, 0.404, 0.550))),
                diffuse: Some(0.2),
                ambient: Some(0.0),
                specular: Some(1.0),
                shininess: Some(200.0),
                reflective: Some(0.7),
                transparency: Some(0.7),
                refractive: Some(1.5),
            }),
            transform: vec![Left("large-object".into())],
            casts_shadow: true,
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
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialSource::Define("white-material".into()),
            transform: vec![
                Left("medium-object".into()),
                Right(Transformation::Translate {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0
                })
            ],
            casts_shadow: true,
        }
    );
}

#[test]
fn should_parse_an_object_with_no_material() {
    let input = "\
add: cube
transform:
  - [ translate, 1, 1, 1 ]
  - [ scale, 3.73335, 2.5845, 1.6283 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialSource::Undefined,
            transform: vec![
                Right(Transformation::Translate {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0
                }),
                Right(Transformation::Scale {
                    x: 3.73335,
                    y: 2.5845,
                    z: 1.6283
                })
            ],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_an_object_with_shadows_disabled() {
    let input = "\
add: cube
shadow: false
transform:
  - [ translate, 1, 1, 1 ]
  - [ scale, 3.73335, 2.5845, 1.6283 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialSource::Undefined,
            transform: vec![
                Right(Transformation::Translate {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0
                }),
                Right(Transformation::Scale {
                    x: 3.73335,
                    y: 2.5845,
                    z: 1.6283
                })
            ],
            casts_shadow: false
        }
    );
}

#[test]
fn should_parse_an_object_from_a_file() {
    let input = "\
add: obj
file: dragon.obj
transform:
    - [ translate, 0, 0.1217, 0]
    - [ scale, 0.268, 0.268, 0.268 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::ObjFile {
                file_name: "dragon.obj".into()
            },
            material: MaterialSource::Undefined,
            transform: vec![
                Right(Transformation::Translate {
                    x: 0.0,
                    y: 0.1217,
                    z: 0.0
                }),
                Right(Transformation::Scale {
                    x: 0.268,
                    y: 0.268,
                    z: 0.268
                })
            ],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_cylinder_with_min_max_and_capped() {
    let input = "\
add: cylinder
min: -0.15
max: 0
closed: true
transform: []";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cylinder {
                min: Some(-0.15),
                max: Some(0.0),
                capped: true
            },
            material: MaterialSource::Undefined,
            transform: vec![],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_adding_primitive_with_no_transform() {
    let input = "\
add: cylinder";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cylinder {
                min: None,
                max: None,
                capped: false
            },
            material: MaterialSource::Undefined,
            transform: vec![],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_add_from_define() {
    let input = "\
add: pedestal";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Reference("pedestal".to_string()),
            material: MaterialSource::Undefined,
            transform: vec![],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_add_group_with_single_child() {
    let input = "\
add: group
transform:
  - [ translate, 0, 2, 0 ]
children:
  - add: pedestal";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Group {
                children: vec![ObjectDescription {
                    kind: ObjectKind::Reference("pedestal".to_string()),
                    material: MaterialSource::Undefined,
                    transform: vec![],
                    casts_shadow: true
                }]
            },
            material: MaterialSource::Undefined,
            transform: vec![Right(Transformation::Translate {
                x: 0.0,
                y: 2.0,
                z: 0.0
            })],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_a_group_containing_a_sub_group() {
    let input = "\
add: group
transform:
  - [ translate, 0, 2, 0 ]
children:
  - add: pedestal
  - add: group
    children:
      - add: dragon
        material:
          color: [ 1, 0, 0.1 ]
          ambient: 0.1
          diffuse: 0.6
          specular: 0.3
          shininess: 15
      - add: bbox
        material:
          ambient: 0
          diffuse: 0.4
          specular: 0
          transparency: 0.6
          refractive-index: 1";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let object = yaml.parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Group {
                children: vec![
                    ObjectDescription {
                        kind: ObjectKind::Reference("pedestal".to_string()),
                        material: MaterialSource::Undefined,
                        transform: vec![],
                        casts_shadow: true
                    },
                    ObjectDescription {
                        kind: ObjectKind::Group {
                            children: vec![
                                ObjectDescription {
                                    kind: ObjectKind::Reference("dragon".into()),
                                    material: MaterialSource::Inline(MaterialDescription {
                                        pattern: Some(Left(Colour::new(1.0, 0.0, 0.1))),
                                        diffuse: Some(0.6),
                                        ambient: Some(0.1),
                                        specular: Some(0.3),
                                        shininess: Some(15.0),
                                        ..Default::default()
                                    }),
                                    transform: vec![],
                                    casts_shadow: true
                                },
                                ObjectDescription {
                                    kind: ObjectKind::Reference("bbox".into()),
                                    material: MaterialSource::Inline(MaterialDescription {
                                        diffuse: Some(0.4),
                                        ambient: Some(0.0),
                                        specular: Some(0.0),
                                        transparency: Some(0.6),
                                        refractive: Some(1.0),
                                        ..Default::default()
                                    }),
                                    transform: vec![],
                                    casts_shadow: true
                                }
                            ]
                        },
                        material: MaterialSource::Undefined,
                        transform: vec![],
                        casts_shadow: true
                    }
                ]
            },
            material: MaterialSource::Undefined,
            transform: vec![Right(Transformation::Translate {
                x: 0.0,
                y: 2.0,
                z: 0.0
            })],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_an_object_define() {
    let input = "\
define: raw-bbox
value:
  add: cube
  shadow: false
";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let define = yaml.parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Object {
            name: "raw-bbox".to_string(),
            value: ObjectDescription {
                kind: ObjectKind::Cube,
                material: MaterialSource::Undefined,
                transform: vec![],
                casts_shadow: false
            }
        }
    );
}

#[test]
#[allow(clippy::approx_constant)] // approximation of PI/2 matches the file
fn should_parse_scene_description() {
    let scene = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/scene_descriptions/cover.yml"
    ));

    let output = parse(scene);
    assert!(output.is_ok(), "{}", output.unwrap_err());
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
                        pattern: Some(Left(Colour::WHITE)),
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
                        pattern: Some(Left(Colour::new(0.537, 0.831, 0.914))),
                        ..Default::default()
                    }
                },
                Define::MaterialDef {
                    name: "red-material".into(),
                    extends: Some("white-material".into()),
                    value: MaterialDescription {
                        pattern: Some(Left(Colour::new(0.941, 0.322, 0.388))),
                        ..Default::default()
                    }
                },
                Define::MaterialDef {
                    name: "purple-material".into(),
                    extends: Some("white-material".into()),
                    value: MaterialDescription {
                        pattern: Some(Left(Colour::new(0.373, 0.404, 0.550))),
                        ..Default::default()
                    }
                },
                Define::Transform {
                    name: "standard-transform".into(),
                    value: vec![
                        Right(Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0
                        }),
                        Right(Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5
                        })
                    ]
                },
                Define::Transform {
                    name: "large-object".into(),
                    value: vec![
                        Left("standard-transform".into()),
                        Right(Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5
                        })
                    ]
                },
                Define::Transform {
                    name: "medium-object".into(),
                    value: vec![
                        Left("standard-transform".into()),
                        Right(Transformation::Scale {
                            x: 3.0,
                            y: 3.0,
                            z: 3.0
                        })
                    ]
                },
                Define::Transform {
                    name: "small-object".into(),
                    value: vec![
                        Left("standard-transform".into()),
                        Right(Transformation::Scale {
                            x: 2.0,
                            y: 2.0,
                            z: 2.0
                        })
                    ]
                },
            ],
            objects: vec![
                ObjectDescription {
                    kind: ObjectKind::Plane,
                    material: MaterialSource::Inline(MaterialDescription {
                        pattern: Some(Left(Colour::WHITE)),
                        ambient: Some(1.0),
                        diffuse: Some(0.0),
                        specular: Some(0.0),
                        ..Default::default()
                    }),
                    transform: vec![
                        Right(Transformation::RotationX(1.5707963267948966)),
                        Right(Transformation::Translate {
                            x: 0.0,
                            y: 0.0,
                            z: 500.0
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Sphere,
                    material: MaterialSource::Inline(MaterialDescription {
                        pattern: Some(Left(Colour::new(0.373, 0.404, 0.550))),
                        diffuse: Some(0.2),
                        ambient: Some(0.0),
                        specular: Some(1.0),
                        shininess: Some(200.0),
                        reflective: Some(0.7),
                        transparency: Some(0.7),
                        refractive: Some(1.5),
                    }),
                    transform: vec![Left("large-object".into())],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("medium-object".into()),
                        Right(Transformation::Translate {
                            x: 4.0,
                            y: 0.0,
                            z: 0.0
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("blue-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 8.5,
                            y: 1.5,
                            z: -0.5
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("red-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 0.0,
                            y: 0.0,
                            z: 4.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("small-object".into()),
                        Right(Transformation::Translate {
                            x: 4.0,
                            y: 0.0,
                            z: 4.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("purple-material".into()),
                    transform: vec![
                        Left("medium-object".into()),
                        Right(Transformation::Translate {
                            x: 7.5,
                            y: 0.5,
                            z: 4.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("medium-object".into()),
                        Right(Transformation::Translate {
                            x: -0.25,
                            y: 0.25,
                            z: 8.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("blue-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 4.0,
                            y: 1.0,
                            z: 7.5,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("red-material".into()),
                    transform: vec![
                        Left("medium-object".into()),
                        Right(Transformation::Translate {
                            x: 10.0,
                            y: 2.0,
                            z: 7.5,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("small-object".into()),
                        Right(Transformation::Translate {
                            x: 8.0,
                            y: 2.0,
                            z: 12.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("small-object".into()),
                        Right(Transformation::Translate {
                            x: 20.0,
                            y: 1.0,
                            z: 9.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("blue-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: -0.5,
                            y: -5.0,
                            z: 0.25,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("red-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 4.0,
                            y: -4.0,
                            z: 0.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 8.5,
                            y: -4.0,
                            z: 0.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 0.0,
                            y: -4.0,
                            z: 4.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("purple-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: -0.5,
                            y: -4.5,
                            z: 8.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: 0.0,
                            y: -8.0,
                            z: 4.0,
                        })
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialSource::Define("white-material".into()),
                    transform: vec![
                        Left("large-object".into()),
                        Right(Transformation::Translate {
                            x: -0.5,
                            y: -8.5,
                            z: 8.0,
                        })
                    ],
                    casts_shadow: true,
                },
            ],
        }
    );
}
