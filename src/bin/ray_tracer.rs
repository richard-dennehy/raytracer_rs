extern crate ray_tracer;

use ray_tracer::*;
use std::f64::consts::PI;
use std::num::NonZeroU16;
use std::time::Instant;

#[macro_use]
extern crate nonzero_ext;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(1920u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(1080u16);

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (-Z) down (+Y), and the near side (+Z) up (-Y), therefore rotation in X should normally be negative
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -2.0)));
    world.objects.push(
        Object::plane()
            .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(2.0))
            .with_material(Material {
                pattern: Pattern::solid(Colour::WHITE),
                // ensure the material colour should be 100% white iff light reaches it
                ambient: 0.0,
                diffuse: 1.0,
                specular: 0.0,
                ..Default::default()
            }),
    );
    world.objects.push(
        Object::plane()
            .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(1.0))
            .with_material(Material {
                pattern: Pattern::solid(Colour::BLACK),
                transparency: 0.5,
                // 50% of light should get through anyway
                casts_shadow: true,
                ..Default::default()
            }),
    );

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Transform::view_transform(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(world, camera);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
