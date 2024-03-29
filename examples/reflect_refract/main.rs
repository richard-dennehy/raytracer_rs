use ray_tracer::renderer::Samples;
use ray_tracer::scene::World;
use ray_tracer::{image_writer, renderer, yaml_parser};
use std::path::Path;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/reflect_refract");
    let timer = Instant::now();

    let mut scene = yaml_parser::load(root.join("resources"), "reflect-refract.yml")?;
    scene.override_resolution(1920, 1080);

    let mut world = World::empty();
    scene.objects()?.into_iter().for_each(|obj| world.add(obj));
    world.lights = scene.lights();

    let camera = scene.camera()?;

    let canvas = renderer::render(&world, &camera, &Samples::single(), true);

    let image = image_writer::write(canvas);
    image
        .save(root.join("reflect_refract.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
