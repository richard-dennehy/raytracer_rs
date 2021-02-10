extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::f64::consts::PI;
use std::num::NonZeroU16;
use std::time::Instant;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(800u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(600u16);

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(2.0, 7.0, -4.0)));

    let plane = || {
        Object::plane().with_material(Material {
            pattern: Pattern::checkers(Colour::new(0.9, 0.9, 0.9), Colour::new(0.8, 0.8, 0.8)),
            ..Default::default()
        })
    };

    let red_sphere = || {
        Object::sphere().with_material(Material {
            pattern: Pattern::solid(Colour::RED),
            ..Default::default()
        })
    };

    let floor = plane().with_transform(Matrix4D::rotation_y(PI / 4.0));
    world.objects.push(floor);

    let left_wall = plane().with_transform(
        Matrix4D::rotation_z(PI / 2.0)
            .with_rotation_y(PI / 4.0)
            .with_translation(0.0, 0.0, 10.0),
    );
    world.objects.push(left_wall);

    let right_wall = plane().with_transform(
        Matrix4D::rotation_z(PI / 2.0)
            .with_rotation_y(-PI / 4.0)
            .with_translation(0.0, 0.0, 10.0),
    );
    world.objects.push(right_wall);

    let csg_cube = Object::csg_difference(
        Object::cube().with_material(Material {
            pattern: Pattern::solid(Colour::new(0.9, 0.9, 0.0)),
            ..Default::default()
        }),
        red_sphere().with_transform(Matrix4D::translation(0.5, 0.5, -0.5)),
    )
    .with_transform(Matrix4D::translation(-2.0, 1.0, 0.0));

    world.objects.push(csg_cube);

    let csg_sphere = Object::csg_difference(
        red_sphere(),
        Object::cube()
            .with_transform(
                Matrix4D::rotation_y(PI / 4.0)
                    .with_scaling(1.0, 1.0, 0.9)
                    .with_rotation_y(-PI / 2.0)
                    .with_translation(0.0, 0.0, -1.5),
            )
            .with_material(Material {
                transparency: 1.0,
                pattern: Pattern::solid(Colour::BLACK),
                ..Default::default()
            }),
    )
    .with_transform(Matrix4D::translation(1.0, 1.0, 0.0));
    world.objects.push(csg_sphere);

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(1.0, 3.0, -7.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    println!("Rendered in {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed());

    Ok(())
}
