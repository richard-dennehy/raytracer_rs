extern crate ray_tracer;

use ray_tracer::renderer::Samples;
use ray_tracer::*;
use std::collections::HashMap;
use std::f64::consts::FRAC_PI_3;
use std::fs;
use std::path::Path;
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
        .push(Light::point(Colour::WHITE, Point3D::new(0.0, 10.0, 5.0)));

    let obj_file = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("meshes/suzanne low poly.obj"),
    )
    .unwrap();
    let obj_data = wavefront_parser::parse_obj(&obj_file, HashMap::new());
    let prism = obj_data.to_object()?;

    world.add(prism.with_material(Material {
        kind: MaterialKind::Solid(Colour::greyscale(0.7)),
        ..Default::default()
    }));

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(0.0, 0.0, 5.0),
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
