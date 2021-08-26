use super::*;
use crate::core::{Colour, Point3D, Vector3D};
use crate::scene::{CsgOperator, Light};

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
    let defines = HashMap::new();
    let output = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<CameraDescription>();
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
    let defines = HashMap::new();
    let output = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<Light>();
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
    let defines = HashMap::new();
    let output = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<Light>();
    assert!(output.is_ok(), "{}", output.unwrap_err());
    let output = output.unwrap();

    assert_eq!(
        output,
        Light::point(Colour::greyscale(0.2), Point3D::new(-400.0, 50.0, -10.0))
    );
}

#[test]
fn should_parse_an_area_light() {
    let input = "\
add: light
corner: [-1, 2, 4]
uvec: [2, 0, 0]
vvec: [0, 2, 0]
usteps: 10
vsteps: 10
jitter: true
intensity: [1.5, 1.5, 1.5]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let output = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<Light>();
    assert!(output.is_ok(), "{}", output.unwrap_err());
    let output = output.unwrap();
    assert_eq!(
        output,
        Light::area(
            Colour::greyscale(1.5),
            Point3D::new(-1.0, 2.0, 4.0),
            Vector3D::new(2.0, 0.0, 0.0),
            Vector3D::new(0.0, 2.0, 0.0),
            nonzero_ext::nonzero!(10u8),
            nonzero_ext::nonzero!(10u8),
            DEFAULT_AREA_LIGHT_SEED
        )
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
    let defines = HashMap::new();
    let define = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Solid(Colour::WHITE)),
            diffuse: Some(0.7),
            ambient: Some(0.1),
            specular: Some(0.0),
            shininess: None,
            reflective: Some(0.1),
            transparency: None,
            refractive: None
        })
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
    let mut defines = HashMap::new();
    defines.insert(
        "white-material".into(),
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Solid(Colour::WHITE)),
            diffuse: Some(0.7),
            ambient: Some(0.1),
            specular: Some(0.0),
            shininess: None,
            reflective: Some(0.1),
            transparency: None,
            refractive: None,
        }),
    );

    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Solid(Colour::new(0.537, 0.831, 0.914))),
            diffuse: Some(0.7),
            ambient: Some(0.1),
            specular: Some(0.0),
            shininess: None,
            reflective: Some(0.1),
            transparency: None,
            refractive: None
        })
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
    let defines = HashMap::new();
    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Pattern {
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
            }),
            ambient: Some(0.0),
            diffuse: Some(0.4),
            specular: Some(0.0),
            reflective: Some(0.3),
            ..Default::default()
        })
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
    let defines = HashMap::new();
    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Pattern {
                pattern_type: PatternType::Checkers,
                colours: (Colour::greyscale(0.35), Colour::greyscale(0.65)),
                transforms: None
            }),
            specular: Some(0.0),
            reflective: Some(0.4),
            ..Default::default()
        })
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
    let defines = HashMap::new();
    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Transform(vec![
            Transformation::Translate {
                x: 1.0,
                y: -1.0,
                z: 1.0
            },
            Transformation::Scale {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        ])
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
    let mut defines = HashMap::new();
    defines.insert(
        "standard-transform".into(),
        Define::Transform(vec![
            Transformation::Translate {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            Transformation::Scale {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        ]),
    );
    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Transform(vec![
            Transformation::Translate {
                x: 1.0,
                y: -1.0,
                z: 1.0
            },
            Transformation::Scale {
                x: 0.5,
                y: 0.5,
                z: 0.5
            },
            Transformation::Scale {
                x: 3.5,
                y: 3.5,
                z: 3.5
            }
        ])
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Plane,
            material: MaterialDescription {
                pattern: Some(PatternKind::Solid(Colour::WHITE)),
                ambient: Some(1.0),
                diffuse: Some(0.0),
                specular: Some(0.0),
                ..Default::default()
            },
            transform: vec![
                Transformation::RotationX(1.5707963267948966),
                Transformation::Translate {
                    x: 0.0,
                    y: 0.0,
                    z: 500.0
                }
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
    let mut defines = HashMap::new();
    defines.insert(
        "large-object".into(),
        Define::Transform(vec![
            Transformation::Translate {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            Transformation::Scale {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Transformation::Scale {
                x: 3.5,
                y: 3.5,
                z: 3.5,
            },
        ]),
    );
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Sphere,
            material: MaterialDescription {
                pattern: Some(PatternKind::Solid(Colour::new(0.373, 0.404, 0.550))),
                diffuse: Some(0.2),
                ambient: Some(0.0),
                specular: Some(1.0),
                shininess: Some(200.0),
                reflective: Some(0.7),
                transparency: Some(0.7),
                refractive: Some(1.5),
            },
            transform: vec![
                Transformation::Translate {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0
                },
                Transformation::Scale {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                },
                Transformation::Scale {
                    x: 3.5,
                    y: 3.5,
                    z: 3.5
                }
            ],
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
    let mut defines = HashMap::new();
    defines.insert(
        "medium-object".into(),
        Define::Transform(vec![
            Transformation::Translate {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            Transformation::Scale {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Transformation::Scale {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
        ]),
    );
    defines.insert(
        "white-material".into(),
        Define::Material(MaterialDescription {
            pattern: Some(PatternKind::Solid(Colour::WHITE)),
            diffuse: Some(0.7),
            ambient: Some(0.1),
            specular: Some(0.0),
            shininess: None,
            reflective: Some(0.1),
            transparency: None,
            refractive: None,
        }),
    );

    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialDescription {
                pattern: Some(PatternKind::Solid(Colour::WHITE)),
                diffuse: Some(0.7),
                ambient: Some(0.1),
                specular: Some(0.0),
                shininess: None,
                reflective: Some(0.1),
                transparency: None,
                refractive: None,
            },
            transform: vec![
                Transformation::Translate {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0
                },
                Transformation::Scale {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                },
                Transformation::Scale {
                    x: 3.0,
                    y: 3.0,
                    z: 3.0
                },
                Transformation::Translate {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0
                }
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialDescription::default(),
            transform: vec![
                Transformation::Translate {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0
                },
                Transformation::Scale {
                    x: 3.73335,
                    y: 2.5845,
                    z: 1.6283
                }
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialDescription::default(),
            transform: vec![
                Transformation::Translate {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0
                },
                Transformation::Scale {
                    x: 3.73335,
                    y: 2.5845,
                    z: 1.6283
                }
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::ObjFile {
                file_name: "dragon.obj".into()
            },
            material: MaterialDescription::default(),
            transform: vec![
                Transformation::Translate {
                    x: 0.0,
                    y: 0.1217,
                    z: 0.0
                },
                Transformation::Scale {
                    x: 0.268,
                    y: 0.268,
                    z: 0.268
                }
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
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
            material: MaterialDescription::default(),
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
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
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
            material: MaterialDescription::default(),
            transform: vec![],
            casts_shadow: true
        }
    );
}

#[test]
fn should_parse_csg_difference() {
    let input = "\
add: csg
operation: difference
left:
  type: cube
  transform:
    - [ scale, 1, 0.25, 1 ]
    - [ translate, 1, 0, 1 ]
    - [ rotate-y, 0.7854 ]
    - [ scale, 1, 1, 0.1 ]
right:
  type: cylinder
  min: -0.26
  max: 0.26
  closed: true
  transform:
    - [ scale, 0.8, 1, 0.8 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();
    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Csg {
                operator: CsgOperator::Subtract,
                left: Box::new(ObjectDescription {
                    kind: ObjectKind::Cube,
                    transform: vec![
                        Transformation::Scale {
                            x: 1.0,
                            y: 0.25,
                            z: 1.0
                        },
                        Transformation::Translate {
                            x: 1.0,
                            y: 0.0,
                            z: 1.0
                        },
                        Transformation::RotationY(0.7 + 0.0854), // pls go away clippy
                        Transformation::Scale {
                            x: 1.0,
                            y: 1.0,
                            z: 0.1
                        }
                    ],
                    casts_shadow: true,
                    material: MaterialDescription::default()
                }),
                right: Box::new(ObjectDescription {
                    kind: ObjectKind::Cylinder {
                        min: Some(-0.26),
                        max: Some(0.26),
                        capped: true
                    },
                    transform: vec![Transformation::Scale {
                        x: 0.8,
                        y: 1.0,
                        z: 0.8
                    }],
                    casts_shadow: true,
                    material: MaterialDescription::default()
                })
            },
            transform: vec![],
            casts_shadow: true,
            material: MaterialDescription::default()
        }
    );
}

#[test]
fn should_parse_add_from_define() {
    let input = "\
add: pedestal";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let mut defines = HashMap::new();
    defines.insert(
        "pedestal".into(),
        Define::Object(ObjectDescription {
            kind: ObjectKind::Cylinder {
                min: None,
                max: None,
                capped: false,
            },
            material: Default::default(),
            transform: vec![],
            casts_shadow: true,
        }),
    );

    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
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
            material: Default::default(),
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
  - add: cube";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Group {
                children: vec![ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription::default(),
                    transform: vec![],
                    casts_shadow: true
                }]
            },
            material: MaterialDescription::default(),
            transform: vec![Transformation::Translate {
                x: 0.0,
                y: 2.0,
                z: 0.0
            }],
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
    let mut defines = HashMap::new();
    defines.insert(
        "dragon".into(),
        Define::Object(ObjectDescription {
            kind: ObjectKind::ObjFile {
                file_name: "dragon.obj".into(),
            },
            material: MaterialDescription::default(),
            transform: vec![
                Transformation::Translate {
                    x: 0.0,
                    y: 0.1217,
                    z: 0.0,
                },
                Transformation::Scale {
                    x: 0.268,
                    y: 0.268,
                    z: 0.268,
                },
            ],
            casts_shadow: true,
        }),
    );
    defines.insert(
        "pedestal".into(),
        Define::Object(ObjectDescription {
            kind: ObjectKind::Cylinder {
                min: Some(-0.15),
                max: Some(0.0),
                capped: true,
            },
            material: MaterialDescription::default(),
            transform: vec![],
            casts_shadow: true,
        }),
    );
    defines.insert(
        "bbox".into(),
        Define::Object(ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialDescription::default(),
            transform: vec![],
            casts_shadow: false,
        }),
    );

    let object = ParseState::new(yaml, &defines)
        .with_context("add")
        .parse::<ObjectDescription>();
    assert!(object.is_ok(), "{}", object.unwrap_err());
    let object = object.unwrap();

    assert_eq!(
        object,
        ObjectDescription {
            kind: ObjectKind::Group {
                children: vec![
                    ObjectDescription {
                        kind: ObjectKind::Cylinder {
                            min: Some(-0.15),
                            max: Some(0.0),
                            capped: true
                        },
                        material: MaterialDescription::default(),
                        transform: vec![],
                        casts_shadow: true
                    },
                    ObjectDescription {
                        kind: ObjectKind::Group {
                            children: vec![
                                ObjectDescription {
                                    kind: ObjectKind::ObjFile {
                                        file_name: "dragon.obj".into(),
                                    },
                                    material: MaterialDescription {
                                        pattern: Some(PatternKind::Solid(Colour::new(
                                            1.0, 0.0, 0.1
                                        ))),
                                        diffuse: Some(0.6),
                                        ambient: Some(0.1),
                                        specular: Some(0.3),
                                        shininess: Some(15.0),
                                        ..Default::default()
                                    },
                                    transform: vec![
                                        Transformation::Translate {
                                            x: 0.0,
                                            y: 0.1217,
                                            z: 0.0,
                                        },
                                        Transformation::Scale {
                                            x: 0.268,
                                            y: 0.268,
                                            z: 0.268,
                                        },
                                    ],
                                    casts_shadow: true,
                                },
                                ObjectDescription {
                                    kind: ObjectKind::Cube,
                                    material: MaterialDescription {
                                        diffuse: Some(0.4),
                                        ambient: Some(0.0),
                                        specular: Some(0.0),
                                        transparency: Some(0.6),
                                        refractive: Some(1.0),
                                        ..Default::default()
                                    },
                                    transform: vec![],
                                    casts_shadow: false,
                                }
                            ]
                        },
                        material: MaterialDescription::default(),
                        transform: vec![],
                        casts_shadow: true
                    }
                ]
            },
            material: MaterialDescription::default(),
            transform: vec![Transformation::Translate {
                x: 0.0,
                y: 2.0,
                z: 0.0
            }],
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
    let defines = HashMap::new();
    let define = ParseState::new(yaml, &defines)
        .with_context("define")
        .parse::<Define>();
    assert!(define.is_ok(), "{}", define.unwrap_err());
    let define = define.unwrap();

    assert_eq!(
        define,
        Define::Object(ObjectDescription {
            kind: ObjectKind::Cube,
            material: MaterialDescription::default(),
            transform: vec![],
            casts_shadow: false
        })
    );
}

#[test]
fn should_parse_a_material_with_a_uv_pattern() {
    let input = "\
pattern:
  type: map
  mapping: spherical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Uv {
                uv_type: UvPatternType::Checkers {
                    width: nonzero_ext::nonzero!(16usize),
                    height: nonzero_ext::nonzero!(8usize),
                    primary: Colour::BLACK,
                    secondary: Colour::greyscale(0.5)
                },
                transforms: None,
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_a_material_with_a_uv_image_pattern() {
    let input = "\
pattern:
  type: map
  mapping: planar
  uv_pattern:
    type: image
    file: wood.jpg
  transform:
    - [ scale, 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Uv {
                uv_type: UvPatternType::Image {
                    file_name: "wood.jpg".into()
                },
                transforms: Some(vec![Transformation::Scale {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5
                }])
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_material_with_rings_pattern() {
    let input = "\
pattern:
  type: rings
  colors:
    - [ 1, 1, 0.5 ]
    - [ 1, 1, 0 ]
  transform:
    - [ scale, 0.05, 1, 0.05 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Pattern {
                pattern_type: PatternType::Rings,
                colours: (Colour::new(1.0, 1.0, 0.5), Colour::new(1.0, 1.0, 0.0)),
                transforms: Some(vec![Transformation::Scale {
                    x: 0.05,
                    y: 1.0,
                    z: 0.05
                }])
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_material_with_gradient_pattern() {
    let input = "\
pattern:
  type: gradient
  colors:
    - [ 1, 1, 0.5 ]
    - [ 1, 1, 0 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Pattern {
                pattern_type: PatternType::Gradient,
                colours: (Colour::new(1.0, 1.0, 0.5), Colour::new(1.0, 1.0, 0.0)),
                transforms: None
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_cubic_uv_pattern() {
    let input = "\
pattern:
  type: map
  mapping: cube
  left:
    type: image
    file: negx.ppm
  right:
    type: image
    file: posx.ppm
  front:
    type: image
    file: posz.ppm
  back:
    type: image
    file: negz.ppm
  up:
    type: image
    file: posy.ppm
  down:
    type: image
    file: negy.ppm";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Uv {
                uv_type: UvPatternType::Cube {
                    left: Box::new(UvPatternType::Image {
                        file_name: "negx.ppm".into()
                    }),
                    right: Box::new(UvPatternType::Image {
                        file_name: "posx.ppm".into()
                    }),
                    front: Box::new(UvPatternType::Image {
                        file_name: "posz.ppm".into()
                    }),
                    back: Box::new(UvPatternType::Image {
                        file_name: "negz.ppm".into()
                    }),
                    top: Box::new(UvPatternType::Image {
                        file_name: "posy.ppm".into()
                    }),
                    bottom: Box::new(UvPatternType::Image {
                        file_name: "negy.ppm".into()
                    }),
                },
                transforms: None
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_cylinder_with_single_uv_pattern() {
    let input = "\
pattern:
  type: map
  mapping: cylindrical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Uv {
                uv_type: UvPatternType::Cylindrical {
                    sides: Box::new(UvPatternType::Checkers {
                        width: nonzero_ext::nonzero!(16usize),
                        height: nonzero_ext::nonzero!(8usize),
                        primary: Colour::BLACK,
                        secondary: Colour::greyscale(0.5)
                    }),
                    caps: None,
                },
                transforms: None
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_parse_cylinder_with_uv_pattern_on_top_and_bottom_caps() {
    let input = "\
pattern:
  type: map
  mapping: cylindrical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]
  top:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]
  bottom:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(material.is_ok(), "{}", material.unwrap_err());
    let material = material.unwrap();

    fn checkers_pattern() -> UvPatternType {
        UvPatternType::Checkers {
            width: nonzero_ext::nonzero!(16usize),
            height: nonzero_ext::nonzero!(8usize),
            primary: Colour::BLACK,
            secondary: Colour::greyscale(0.5),
        }
    }

    assert_eq!(
        material,
        MaterialDescription {
            pattern: Some(PatternKind::Uv {
                uv_type: UvPatternType::Cylindrical {
                    sides: Box::new(checkers_pattern()),
                    caps: Some((Box::new(checkers_pattern()), Box::new(checkers_pattern()))),
                },
                transforms: None
            }),
            ..Default::default()
        }
    );
}

#[test]
fn should_not_parse_cylinder_with_uv_pattern_on_top_but_not_bottom_cap() {
    let input = "\
pattern:
  type: map
  mapping: cylindrical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]
  top:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(
        material.is_err(),
        "expected parsing to fail, but it succeeded"
    );
    assert_eq!(
        &format!("{:?}", material.unwrap_err()),
        "cannot parse `material` as Material

Caused by:
    0: cannot parse `pattern` as Pattern (Optional)
    1: cannot parse `pattern` as UV pattern
    2: a cylindrical map with a `top` pattern must also have a `bottom` pattern"
    );
}

#[test]
fn should_not_parse_cylinder_with_uv_pattern_on_bottom_but_not_top_cap() {
    let input = "\
pattern:
  type: map
  mapping: cylindrical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]
  bottom:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]";

    let yaml = &YamlLoader::load_from_str(input).unwrap()[0];
    let defines = HashMap::new();
    let material = ParseState::new(yaml, &defines)
        .with_context("material")
        .parse::<MaterialDescription>();
    assert!(
        material.is_err(),
        "expected parsing to fail, but it succeeded"
    );
    assert_eq!(
        &format!("{:?}", material.unwrap_err()),
        "cannot parse `material` as Material

Caused by:
    0: cannot parse `pattern` as Pattern (Optional)
    1: cannot parse `pattern` as UV pattern
    2: a cylindrical map with a `bottom` pattern must also have a `top` pattern"
    );
}

#[test]
#[allow(clippy::approx_constant)] // approximation of PI/2 matches the file
fn should_parse_scene_description() {
    let scene = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/cover/resources/cover.yml"
    ));

    let output = parse(scene, Default::default());
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
                Light::point(Colour::WHITE, Point3D::new(50.0, 100.0, -50.0)),
                Light::point(Colour::greyscale(0.2), Point3D::new(-400.0, 50.0, -10.0)),
            ],
            objects: vec![
                ObjectDescription {
                    kind: ObjectKind::Plane,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(1.0),
                        diffuse: Some(0.0),
                        specular: Some(0.0),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::RotationX(1.5707963267948966),
                        Transformation::Translate {
                            x: 0.0,
                            y: 0.0,
                            z: 500.0
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Sphere,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.373, 0.404, 0.550))),
                        diffuse: Some(0.2),
                        ambient: Some(0.0),
                        specular: Some(1.0),
                        shininess: Some(200.0),
                        reflective: Some(0.7),
                        transparency: Some(0.7),
                        refractive: Some(1.5),
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.0,
                            y: 3.0,
                            z: 3.0,
                        },
                        Transformation::Translate {
                            x: 4.0,
                            y: 0.0,
                            z: 0.0
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.537, 0.831, 0.914))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 8.5,
                            y: 1.5,
                            z: -0.5
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.941, 0.322, 0.388))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 0.0,
                            y: 0.0,
                            z: 4.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 2.0,
                            y: 2.0,
                            z: 2.0,
                        },
                        Transformation::Translate {
                            x: 4.0,
                            y: 0.0,
                            z: 4.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.373, 0.404, 0.55))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.0,
                            y: 3.0,
                            z: 3.0,
                        },
                        Transformation::Translate {
                            x: 7.5,
                            y: 0.5,
                            z: 4.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.0,
                            y: 3.0,
                            z: 3.0,
                        },
                        Transformation::Translate {
                            x: -0.25,
                            y: 0.25,
                            z: 8.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.537, 0.831, 0.914))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 4.0,
                            y: 1.0,
                            z: 7.5,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.941, 0.322, 0.388))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.0,
                            y: 3.0,
                            z: 3.0,
                        },
                        Transformation::Translate {
                            x: 10.0,
                            y: 2.0,
                            z: 7.5,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 2.0,
                            y: 2.0,
                            z: 2.0,
                        },
                        Transformation::Translate {
                            x: 8.0,
                            y: 2.0,
                            z: 12.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 2.0,
                            y: 2.0,
                            z: 2.0,
                        },
                        Transformation::Translate {
                            x: 20.0,
                            y: 1.0,
                            z: 9.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.537, 0.831, 0.914))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: -0.5,
                            y: -5.0,
                            z: 0.25,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.941, 0.322, 0.388))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 4.0,
                            y: -4.0,
                            z: 0.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 8.5,
                            y: -4.0,
                            z: 0.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 0.0,
                            y: -4.0,
                            z: 4.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::new(0.373, 0.404, 0.55))),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: -0.5,
                            y: -4.5,
                            z: 8.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: 0.0,
                            y: -8.0,
                            z: 4.0,
                        }
                    ],
                    casts_shadow: true,
                },
                ObjectDescription {
                    kind: ObjectKind::Cube,
                    material: MaterialDescription {
                        pattern: Some(PatternKind::Solid(Colour::WHITE)),
                        ambient: Some(0.1),
                        diffuse: Some(0.7),
                        specular: Some(0.0),
                        reflective: Some(0.1),
                        ..Default::default()
                    },
                    transform: vec![
                        Transformation::Translate {
                            x: 1.0,
                            y: -1.0,
                            z: 1.0,
                        },
                        Transformation::Scale {
                            x: 0.5,
                            y: 0.5,
                            z: 0.5,
                        },
                        Transformation::Scale {
                            x: 3.5,
                            y: 3.5,
                            z: 3.5,
                        },
                        Transformation::Translate {
                            x: -0.5,
                            y: -8.5,
                            z: 8.0,
                        }
                    ],
                    casts_shadow: true,
                },
            ],
            resource_dir: Default::default()
        }
    );
}
