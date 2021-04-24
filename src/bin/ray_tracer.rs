extern crate ray_tracer;

use ray_tracer::renderer::Subsamples;
use ray_tracer::*;
use std::time::Instant;

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (+Z) down (+Y), and the near side (-Z) up (-Y), therefore rotation in X should normally be negative
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();

    world.lights.push(Light::point(
        Colour::WHITE,
        Point3D::new(-10.0, 100.0, -100.0),
    ));

    let pedestal = Object::cylinder()
        .min_y(-0.15)
        .max_y(0.0)
        .capped()
        .build()
        .with_material(Material {
            pattern: Pattern::solid(Colour::RED),
            ambient: 0.0,
            specular: 0.0,
            diffuse: 1.0,
            ..Default::default()
        });
    world.add(pedestal);

    let glass_box = Object::cube()
        .with_material(Material {
            pattern: Pattern::solid(Colour::greyscale(0.1)),
            casts_shadow: false,
            ambient: 0.0,
            diffuse: 1.0,
            specular: 0.0,
            transparency: 0.9,
            refractive: 1.0,
            ..Default::default()
        })
        .transformed(
            Transform::identity()
                .scale_x(0.5002689)
                .scale_y(0.346323)
                .scale_z(0.2181922)
                .translate_x(-0.0338752)
                .translate_y(0.346323)
                .translate_z(0.0598042),
        );

    world.add(glass_box);

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        1.2,
        Transform::view_transform(
            Point3D::new(0.0, 2.5, -10.0),
            Point3D::new(0.0, 1.0, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(world, camera, Subsamples::None);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
