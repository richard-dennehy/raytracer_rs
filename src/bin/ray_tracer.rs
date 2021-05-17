extern crate ray_tracer;

use ray_tracer::renderer::Samples;
use ray_tracer::*;
use std::f64::consts::PI;
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
        .push(Light::point(Colour::WHITE, Point3D::new(10.0, 10.0, -10.0)));

    world.add(Object::cube().with_material(Material {
        kind: MaterialKind::Uv(UvPattern::cubic(
            UvPattern::alignment_check(
                Colour::new(0.0, 1.0, 1.0),
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(1.0, 0.5, 0.0),
                Colour::GREEN,
            ),
            UvPattern::alignment_check(
                Colour::GREEN,
                Colour::new(1.0, 0.0, 1.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::WHITE,
                Colour::BLUE,
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::RED,
                Colour::BLUE,
                Colour::new(1.0, 0.5, 0.0),
            ),
            UvPattern::alignment_check(
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
                Colour::new(1.0, 0.0, 1.0),
                Colour::GREEN,
                Colour::WHITE,
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 0.5, 0.0),
                Colour::new(0.0, 1.0, 1.0),
                Colour::new(1.0, 0.0, 1.0),
                Colour::RED,
                Colour::new(1.0, 1.0, 0.0),
            ),
            UvPattern::alignment_check(
                Colour::new(1.0, 0.0, 1.0),
                Colour::new(1.0, 0.5, 0.0),
                Colour::GREEN,
                Colour::BLUE,
                Colour::WHITE,
            ),
        )),
        ..Default::default()
    }));

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        PI / 3.0,
        Transform::view_transform(
            Point3D::new(3.0, 3.0, -5.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(world, camera, &Samples::single());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
