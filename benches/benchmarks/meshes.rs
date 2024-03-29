use criterion::{criterion_group, Criterion};
use ray_tracer::core::*;
use ray_tracer::renderer::{render, Camera, Samples};
use ray_tracer::scene::{Light, World};
use ray_tracer::wavefront_parser::WavefrontParser;
use std::f64::consts::FRAC_PI_3;
use std::path::Path;

criterion_group! {
    benches,
    basic_triangle_meshes,
    complex_meshes,
    very_complex_meshes
}

fn basic_triangle_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic meshes (800x600)");

    for file_name in ["prism flat.obj", "prism smooth.obj"] {
        let parser = WavefrontParser::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("meshes"));
        let _ = parser.load(file_name).unwrap();

        group.bench_with_input(file_name, &parser, |b, parser| {
            let prism = parser.load(file_name).unwrap();

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

            b.iter(|| render(&world, &camera, &Samples::single(), false))
        });
    }
}

fn complex_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex meshes (600x600)");

    for file_name in ["suzanne low poly.obj", "suzanne lp smooth.obj"] {
        let parser = WavefrontParser::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("meshes"));
        let _ = parser.load(file_name).unwrap();

        group.bench_with_input(file_name, &parser, |b, parser| {
            let prism = parser.load(file_name).unwrap();

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

            b.iter(|| render(&world, &camera, &Samples::single(), false))
        });
    }
}

fn very_complex_meshes(c: &mut Criterion) {
    let mut group = c.benchmark_group("very complex meshes (300x300)");
    group.sample_size(20);

    for file_name in [
        "suzanne medium poly.obj",
        "suzanne high poly.obj",
        "suzanne mp smooth.obj",
    ] {
        let parser = WavefrontParser::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("meshes"));
        let _ = parser.load(file_name).unwrap();

        group.bench_with_input(file_name, &parser, |b, parser| {
            let prism = parser.load(file_name).unwrap();

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

            b.iter(|| render(&world, &camera, &Samples::single(), false))
        });
    }
}
