use criterion::{criterion_group, Criterion};
use ray_tracer::renderer::{render, Samples};
use ray_tracer::{wavefront_parser, Camera, Colour, Light, Normal3D, Point3D, Transform, World};
use std::f64::consts::FRAC_PI_3;
use std::fs;
use std::path::Path;

criterion_group! {
    benches,
    basic_triangle_meshes,
    complex_meshes,
    very_complex_meshes
}

fn basic_triangle_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic meshes (800x600)");

    for file_name in ["prism flat", "prism smooth"] {
        let file = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("meshes/{}.obj", file_name)),
        )
        .expect(&format!("failed to read mesh file {}", file_name));

        let obj = wavefront_parser::parse_obj(&file);

        group.bench_with_input(file_name, &obj, |b, obj| {
            b.iter(|| {
                let prism = obj.to_object().unwrap();

                let mut world = World::empty();
                world.add(prism);
                world
                    .lights
                    .push(Light::point(Colour::WHITE, Point3D::new(10.0, 10.0, 0.0)));

                let camera = Camera::new(
                    nonzero_ext::nonzero!(800u16),
                    nonzero_ext::nonzero!(600u16),
                    FRAC_PI_3,
                    Transform::view_transform(
                        Point3D::new(0.0, 0.0, 5.0),
                        Point3D::ORIGIN,
                        Normal3D::POSITIVE_Y,
                    ),
                );

                render(world, camera, &Samples::single())
            })
        });
    }
}

fn complex_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex meshes (600x600)");

    for file_name in ["suzanne low poly", "suzanne lp smooth"] {
        let file = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("meshes/{}.obj", file_name)),
        )
        .expect(&format!("failed to read mesh file {}", file_name));

        let obj = wavefront_parser::parse_obj(&file);

        group.bench_with_input(file_name, &obj, |b, obj| {
            b.iter(|| {
                let prism = obj.to_object().unwrap();

                let mut world = World::empty();
                world.add(prism);
                world
                    .lights
                    .push(Light::point(Colour::WHITE, Point3D::new(10.0, 10.0, 0.0)));

                let camera = Camera::new(
                    nonzero_ext::nonzero!(600u16),
                    nonzero_ext::nonzero!(600u16),
                    FRAC_PI_3,
                    Transform::view_transform(
                        Point3D::new(0.0, 0.0, 5.0),
                        Point3D::ORIGIN,
                        Normal3D::POSITIVE_Y,
                    ),
                );

                render(world, camera, &Samples::single())
            })
        });
    }
}

fn very_complex_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("very complex meshes (300x300)");
    group.sample_size(20);

    for file_name in [
        "suzanne medium poly",
        "suzanne high poly",
        "suzanne mp smooth",
    ] {
        let file = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("meshes/{}.obj", file_name)),
        )
        .expect(&format!("failed to read mesh file {}", file_name));

        let obj = wavefront_parser::parse_obj(&file);

        group.bench_with_input(file_name, &obj, |b, obj| {
            b.iter(|| {
                let prism = obj.to_object().unwrap();

                let mut world = World::empty();
                world.add(prism);
                world
                    .lights
                    .push(Light::point(Colour::WHITE, Point3D::new(10.0, 10.0, 0.0)));

                let camera = Camera::new(
                    nonzero_ext::nonzero!(400u16),
                    nonzero_ext::nonzero!(400u16),
                    FRAC_PI_3,
                    Transform::view_transform(
                        Point3D::new(0.0, 0.0, 5.0),
                        Point3D::ORIGIN,
                        Normal3D::POSITIVE_Y,
                    ),
                );

                render(world, camera, &Samples::single())
            })
        });
    }
}
