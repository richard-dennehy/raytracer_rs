use image::imageops::FilterType;
use ray_tracer::renderer::Samples;
use ray_tracer::scene::World;
use ray_tracer::{image_writer, renderer, yaml_parser};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/dragons");
    let timer = Instant::now();

    let mut scene = yaml_parser::load(root.join("resources"), "bounding-boxes.yml")?;
    scene.override_resolution(7680, 4320);

    let mut world = World::empty();
    scene.objects()?.into_iter().for_each(|obj| world.add(obj));
    world.lights = scene.lights();

    let camera = scene.camera()?;

    let canvas = renderer::render(
        &world,
        &camera,
        &Samples::grid(nonzero_ext::nonzero!(4u8)),
        true,
    );

    let original = image_writer::write(canvas);
    let resized = image::imageops::resize(&original, 1920, 1080, FilterType::Gaussian);
    resized
        .save(root.join("dragons_supersampled.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
