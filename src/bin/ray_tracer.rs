extern crate ray_tracer;

use ray_tracer::*;
use std::f64::consts::PI;
use std::time::Instant;

#[macro_use]
extern crate nonzero_ext;

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world.lights.push(Light::point(
        Colour::WHITE,
        Point3D::new(-10.0, 10.0, -10.0),
    ));
    world
        .objects
        .push(Object::sphere().with_transform(Matrix4D::translation(0.0, 0.0, 0.5)));

    let camera = Camera::new(
        nonzero!(200u16),
        nonzero!(200u16),
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, -5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );

    let canvas = renderer::render(world, camera);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
