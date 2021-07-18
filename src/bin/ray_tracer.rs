extern crate ray_tracer;

use ray_tracer::renderer::Samples;
use ray_tracer::*;
use std::f64::consts::{FRAC_PI_3, PI};
use std::time::Instant;

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (+Z) down (-Y), and the near side (-Z) up (+Y), therefore rotation in X should normally be negative
/// Rotation in positive Y moves the right side (+X) closer to the camera (-Z) and the left side (-X) further from the camera (+Z)

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));

    world.add(
        Object::cone()
            .min_y(-3.0)
            .max_y(-1.0)
            .capped()
            .build()
            .transformed(
                Transform::identity()
                    .rotate_x(PI)
                    .translate_x(-2.0)
                    .translate_y(-2.0),
            ),
    );

    world.add(
        Object::cone()
            .min_y(0.0)
            .max_y(2.0)
            .capped()
            .build()
            .transformed(Transform::identity().translate_x(4.0).translate_y(-1.0)),
    );

    world.add(
        Object::cone()
            .min_y(-0.75)
            .max_y(0.75)
            .capped()
            .build()
            .transformed(Transform::identity().translate_x(1.0).translate_z(-3.0)),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(2.0, 4.0, -10.0),
            Point3D::new(1.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(world, camera, &Samples::single());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
