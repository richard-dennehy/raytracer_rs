use criterion::{criterion_group, BenchmarkId, Criterion};
use ray_tracer::{renderer, yaml_parser, World};
use std::fs;
use std::path::Path;

criterion_group! {
    benches,
    cover_image,
    reflect_refract,
}

fn cover_image(c: &mut Criterion) {
    let mut group = c.benchmark_group("render cover image from YAML");
    group.sample_size(10);

    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scene_descriptions/cover.yml"),
    )
    .unwrap();

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut scene = yaml_parser::parse(&yaml).unwrap();
                    scene.override_resolution(*x as _, *y as _);

                    let mut world = World::empty();
                    world.lights.append(&mut scene.lights());
                    world.objects.append(&mut scene.objects().unwrap());

                    let camera = scene.camera().unwrap();

                    renderer::render(world, camera);
                })
            },
        );
    }
}

fn reflect_refract(c: &mut Criterion) {
    let mut group = c.benchmark_group("render reflection + reflection image from YAML");
    group.sample_size(10);

    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scene_descriptions/reflect-refract.yml"),
    )
    .unwrap();

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut scene = yaml_parser::parse(&yaml).unwrap();
                    scene.override_resolution(*x as _, *y as _);

                    let mut world = World::empty();
                    world.lights.append(&mut scene.lights());
                    world.objects.append(&mut scene.objects().unwrap());

                    let camera = scene.camera().unwrap();

                    renderer::render(world, camera);
                })
            },
        );
    }
}

const RESOLUTIONS: [(u16, u16); 3] = [(200, 200), (400, 400), (600, 600)];
