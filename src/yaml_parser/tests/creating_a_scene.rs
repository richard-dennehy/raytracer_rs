use super::*;
use crate::core::{Colour, Point3D, Transform, Vector3D, VectorMaths};
use crate::renderer::Camera;
use crate::scene::{Material, MaterialKind, Pattern};
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

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let camera = scene.camera();
    assert!(camera.is_ok(), "{}", camera.unwrap_err());
    let camera = camera.unwrap();

    assert_eq!(
        camera,
        Camera::new(
            nonzero!(100_u16),
            nonzero!(100_u16),
            0.785,
            Transform::view_transform(
                Point3D::new(-6.0, 6.0, -10.0),
                Point3D::new(6.0, 0.0, 6.0),
                Vector3D::new(-0.45, 1.0, 0.0).normalised()
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

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
fn should_be_able_to_create_an_object_with_a_checker_pattern() {
    let input = with_camera_description(
        "\
- add: plane
  transform: []
  material:
    pattern:
      type: checkers
      colors:
        - [0.35, 0.35, 0.35]
        - [0.65, 0.65, 0.65]
    specular: 0
    reflective: 0.4",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Plane");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Pattern(Pattern::checkers(
                Colour::greyscale(0.35),
                Colour::greyscale(0.65)
            )),
            specular: 0.0,
            reflective: 0.4,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
#[allow(clippy::approx_constant)]
fn should_be_able_to_create_an_object_with_a_pattern_with_a_transform() {
    let input = with_camera_description(
        "\
- add: plane
  transform: []
  material:
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
    reflective: 0.3",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Plane");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Pattern(
                Pattern::striped(Colour::greyscale(0.45), Colour::greyscale(0.55))
                    .with_transform(Transform::identity().rotate_y(1.5708).scale_all(0.25))
            ),
            ambient: 0.0,
            diffuse: 0.4,
            specular: 0.0,
            reflective: 0.3,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
fn should_be_able_to_create_an_object_referencing_a_defined_material() {
    let input = with_camera_description(
        "\
- define: white-material
  value:
    color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1

- add: sphere
  material: white-material
  transform: []",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::WHITE),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
fn should_be_to_create_an_object_with_a_material_extending_another_material() {
    let input = with_camera_description(
        "\
- define: white-material
  value:
    color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1

- define: blue-material
  extend: white-material
  value:
    color: [ 0.537, 0.831, 0.914 ]

- add: cube
  material: blue-material
  transform: []",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Cube");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.537, 0.831, 0.914)),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
fn should_create_an_object_with_a_material_extending_a_material_extending_another_material() {
    let input = with_camera_description(
        "\
- define: white-material
  value:
    color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1

- define: blue-material
  extend: white-material
  value:
    color: [ 0.537, 0.831, 0.914 ]

- define: transparent-blue-material
  extend: blue-material
  value:
    transparency: 0.9

- add: plane
  material: transparent-blue-material
  transform: []",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Plane");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.537, 0.831, 0.914)),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            transparency: 0.9,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Transform::identity());
}

#[test]
fn should_be_able_to_create_an_object_with_a_single_transform() {
    let input = with_camera_description(
        "\
- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform:
    - [ translate, 1, 0, 0 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(
        objects[0].transform(),
        Transform::identity().translate_x(1.0)
    );
}

#[test]
fn should_be_able_to_create_an_object_with_multiple_transforms() {
    let input = with_camera_description(
        "\
- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform:
  - [ translate, 1, 0, 0 ]
  - [ scale, 1, 2, 1 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(
        objects[0].transform(),
        Transform::identity().translate_x(1.0).scale_y(2.0)
    );
}

#[test]
fn should_be_able_to_create_an_object_referencing_a_defined_transform() {
    let input = with_camera_description(
        "\
- define: standard-transform
  value:
    - [ translate, 1, -1, 1 ]
    - [ scale, 0.5, 0.5, 0.5 ]

- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform:
    - standard-transform",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(
        objects[0].transform(),
        Transform::identity()
            .translate_x(1.0)
            .translate_y(-1.0)
            .translate_z(1.0)
            .scale_all(0.5)
    );
}

#[test]
fn should_be_able_to_create_an_object_extending_a_defined_transform() {
    let input = with_camera_description(
        "\
- define: standard-transform
  value:
    - [ translate, 1, -1, 1 ]
    - [ scale, 0.5, 0.5, 0.5 ]

- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform:
    - standard-transform
    - [ translate, 4, 0, 0 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(
        objects[0].transform(),
        Transform::identity()
            .translate_x(1.0)
            .translate_y(-1.0)
            .translate_z(1.0)
            .scale_all(0.5)
            .translate_x(4.0)
    );
}

#[test]
fn should_be_able_to_create_an_object_extending_a_transform_extending_another_transform() {
    let input = with_camera_description(
        "\
- define: standard-transform
  value:
    - [ translate, 1, -1, 1 ]
    - [ scale, 0.5, 0.5, 0.5 ]

- define: medium-object
  value:
    - standard-transform
    - [ scale, 3, 3, 3 ]

- add: sphere
  material:
    color: [ 0.373, 0.404, 0.550 ]
  transform:
    - medium-object
    - [ translate, 4, 0, 0 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Sphere");
    assert_eq!(
        objects[0].material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(
        objects[0].transform(),
        Transform::identity()
            .translate_x(1.0)
            .translate_y(-1.0)
            .translate_z(1.0)
            .scale_all(0.5)
            .scale_all(3.0)
            .translate_x(4.0)
    );
}

#[test]
fn should_be_able_to_create_a_group() {
    let input = with_camera_description(
        "\
- add: group
  transform:
    - [ translate, 0, 2, 0 ]
  children:
    - add: sphere
      material:
        color: [ 0.373, 0.404, 0.550 ]
    - add: cube
      material:
        color: [ 0.373, 0.404, 0.550 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    let children = objects[0].children();
    assert_eq!(children.len(), 2);

    let sphere = &children[0];
    assert_eq!(format!("{:?}", sphere.shape()), "Sphere");

    let cube = &children[1];
    assert_eq!(format!("{:?}", cube.shape()), "Cube");

    assert_eq!(
        sphere.material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(sphere.transform(), Transform::identity().translate_y(2.0));

    assert_eq!(
        cube.material,
        Material {
            kind: MaterialKind::Solid(Colour::new(0.373, 0.404, 0.55)),
            ..Default::default()
        }
    );
    assert_eq!(cube.transform(), Transform::identity().translate_y(2.0));
}

#[test]
fn should_be_able_to_create_a_group_with_a_subgroup() {
    let input = with_camera_description(
        "\
- add: group
  children:
    - add: sphere
    - add: group
      children:
        - add: sphere",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    let children = objects[0].children();
    assert_eq!(children.len(), 2);

    let sphere = &children[0];
    assert_eq!(format!("{:?}", sphere.shape()), "Sphere");

    let subgroup = &children[1];
    assert_eq!(subgroup.children().len(), 1);

    let sphere = &children[1].children()[0];
    assert_eq!(format!("{:?}", sphere.shape()), "Sphere");
}

#[test]
fn should_be_able_to_add_group_from_define() {
    let input = with_camera_description(
        "\
- define: raw-bbox
  value:
    add: cube
    shadow: false
    transform:
      - [ translate, 1, 1, 1 ]

- add: raw-bbox",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    let cube = &objects[0];
    assert_eq!(format!("{:?}", cube.shape()), "Cube");
    assert_eq!(
        cube.transform(),
        Transform::identity()
            .translate_x(1.0)
            .translate_y(1.0)
            .translate_z(1.0)
    );
    assert_eq!(cube.material.casts_shadow, false);
}

#[test]
fn should_be_able_to_add_an_object_from_a_define_and_override_the_material() {
    let input = with_camera_description(
        "\
- define: raw-bbox
  value:
    add: cube
    shadow: false
    transform:
      - [ translate, 1, 1, 1 ]

- add: raw-bbox
  material:
    color: [ 1, 0, 0 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    let cube = &objects[0];
    assert_eq!(format!("{:?}", cube.shape()), "Cube");
    assert_eq!(
        cube.material,
        Material {
            kind: MaterialKind::Solid(Colour::RED),
            casts_shadow: false,
            ..Default::default()
        }
    );
}

#[test]
fn should_be_able_to_create_a_csg() {
    let input = with_camera_description(
        "\
- add: csg
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
      - [ scale, 0.8, 1, 0.8 ]",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();
    assert_eq!(objects.len(), 1);

    let (left, right) = objects[0].csg_children();
    assert_eq!(format!("{:?}", left.shape()), "Cube");
    approx::assert_abs_diff_eq!(
        left.transform(),
        Transform::identity()
            .scale_y(0.25)
            .translate_x(1.0)
            .translate_z(1.0)
            .rotate_y(0.785 + 0.004)
            .scale_z(0.1),
        epsilon = 0.05 // rounding errors are absurd in this instance
    );

    assert_eq!(
        format!("{:?}", right.shape()),
        "Cylinder { max_y: 0.26, min_y: -0.26, capped: true }"
    );
    assert_eq!(
        right.transform(),
        Transform::identity().scale_x(0.8).scale_z(0.8)
    );
}

#[test]
fn should_be_able_to_create_a_cone() {
    let input = with_camera_description(
        "\
- add: cone
  min: -1.0
  max: 0.0
  closed: true",
    );

    let scene = parse(&input, Default::default());
    assert!(scene.is_ok(), "{}", scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), "{}", objects.unwrap_err());
    let objects = objects.unwrap();
    assert_eq!(objects.len(), 1);

    assert_eq!(
        format!("{:?}", objects[0].shape()),
        "Cone { max_y: 0.0, min_y: -1.0, capped: true }"
    );
}
