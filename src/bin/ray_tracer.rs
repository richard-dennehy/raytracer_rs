extern crate ray_tracer;

use image::imageops::FilterType;
use ray_tracer::renderer::Samples;
use ray_tracer::*;
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

    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scene_descriptions/bounding-boxes.yml"),
    )
    .unwrap();

    let mut scene = yaml_parser::parse(&yaml).unwrap();
    scene.override_resolution(3840, 2160);

    let mut world = World::empty();
    world.lights.append(&mut scene.lights());
    world.add(Object::group(scene.objects().unwrap()));

    let camera = scene.camera().unwrap();

    let canvas = renderer::render(world, camera, &Samples::grid(nonzero_ext::nonzero!(3u8)));

    println!("Rendered at {:.2?}", timer.elapsed());

    let supersampled = image_writer::write(canvas);
    let resized = image::imageops::resize(&supersampled, 1920, 1080, FilterType::Nearest);
    resized
        .save("out.png")
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
