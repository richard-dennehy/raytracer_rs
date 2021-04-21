extern crate ray_tracer;

use nonzero_ext::nonzero;
use ray_tracer::*;
use std::f64::consts::PI;
use std::time::Instant;

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
        .push(Light::point(Colour::WHITE, Point3D::new(10.0, 5.0, 0.0)));

    world.add(Object::plane().with_material(Material {
        pattern: Pattern::checkers(Colour::BLACK, Colour::WHITE),
        ..Default::default()
    }));

    let camera = Camera::new(
        nonzero!(1920u16),
        nonzero!(1080u16),
        PI / 3.0,
        Transform::view_transform(
            Point3D::new(0.0, 5.0, 0.0),
            Point3D::new(0.0, 0.0, 10.0),
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(world, camera, nonzero!(64u8));

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
