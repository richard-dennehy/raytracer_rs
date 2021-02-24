use super::*;
use crate::{Camera, Colour, Material, Matrix4D, Pattern, Point3D, Vector3D};
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
            pattern: Pattern::solid(Colour::WHITE),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Matrix4D::identity());
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

    let scene = parse(&input);
    assert!(scene.is_ok(), scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Cube");
    assert_eq!(
        objects[0].material,
        Material {
            pattern: Pattern::solid(Colour::new(0.537, 0.831, 0.914)),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Matrix4D::identity());
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

    let scene = parse(&input);
    assert!(scene.is_ok(), scene.unwrap_err());
    let scene = scene.unwrap();

    let objects = scene.objects();
    assert!(objects.is_ok(), objects.unwrap_err());
    let objects = objects.unwrap();

    assert_eq!(objects.len(), 1);
    assert_eq!(format!("{:?}", objects[0].shape()), "Plane");
    assert_eq!(
        objects[0].material,
        Material {
            pattern: Pattern::solid(Colour::new(0.537, 0.831, 0.914)),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.0,
            reflective: 0.1,
            transparency: 0.9,
            ..Default::default()
        }
    );
    assert_eq!(objects[0].transform(), Matrix4D::identity());
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
    assert_eq!(objects[0].transform(), Matrix4D::translation(1.0, 0.0, 0.0));
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
    assert_eq!(
        objects[0].transform(),
        Matrix4D::scaling(1.0, 2.0, 1.0).with_translation(1.0, 0.0, 0.0)
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
    assert_eq!(
        objects[0].transform(),
        Matrix4D::translation(1.0, -1.0, 1.0).with_scaling(0.5, 0.5, 0.5)
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
    assert_eq!(
        objects[0].transform(),
        Matrix4D::translation(1.0, -1.0, 1.0)
            .with_scaling(0.5, 0.5, 0.5)
            .with_translation(4.0, 0.0, 0.0)
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
    assert_eq!(
        objects[0].transform(),
        Matrix4D::translation(1.0, -1.0, 1.0)
            .with_scaling(0.5, 0.5, 0.5)
            .with_scaling(3.0, 3.0, 3.0)
            .with_translation(4.0, 0.0, 0.0)
    );
}