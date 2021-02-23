extern crate ray_tracer;

use ray_tracer::*;
use std::fs;
use std::time::Instant;

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();

    let scene_yaml =
        fs::read_to_string("scene_descriptions/cover.yml").map_err(|err| err.to_string())?;
    let scene = yaml_parser::parse(&scene_yaml)?;

    let mut objects = scene.objects()?;
    world.objects.append(&mut objects);
    world.lights.append(&mut scene.lights());

    let camera = scene.camera()?;

    let canvas = renderer::render(world, camera);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
