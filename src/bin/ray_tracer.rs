extern crate ray_tracer;

use ray_tracer::core::{Colour, Normal3D, Point3D, Transform, Vector3D, VectorMaths};
use ray_tracer::renderer::{Camera, Samples};
use ray_tracer::scene::{Light, Material, MaterialKind, Object, Pattern, World};
use ray_tracer::*;
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_3};
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
        .push(Light::point(Colour::WHITE, Point3D::new(-5.0, 0.0, -5.0)));
    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::checkers(Colour::BLACK, Colour::WHITE)),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_y(-1.0)),
    );
    world.add(Object::smooth_triangle(
        Point3D::ORIGIN,
        Point3D::new(0.0, 1.0, 0.0),
        Point3D::new(1.0, 0.0, 0.0),
        Vector3D::new(-FRAC_1_SQRT_2, -0.5, -0.5).normalised(),
        Vector3D::new(0.0, 1.7071, -FRAC_1_SQRT_2).normalised(),
        Vector3D::new(1.7071, 0.0, -FRAC_1_SQRT_2).normalised(),
    ));

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(-3.0, 2.0, -2.0),
            Point3D::new(0.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(&world, &camera, &Samples::single(), true);

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
