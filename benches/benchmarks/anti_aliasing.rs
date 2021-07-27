use std::array::IntoIter;
use std::f64::consts::PI;

use criterion::{criterion_group, BenchmarkId, Criterion};

use ray_tracer::core::*;
use ray_tracer::renderer::Samples;
use ray_tracer::{renderer, Camera, Colour, Light, Material, MaterialKind, Object, Pattern, World};

criterion_group! {
    benches,
    single_sphere,
    basic_scene
}

fn single_sphere(c: &mut Criterion) {
    let mut group = c.benchmark_group("single sphere with anti-aliasing");
    group.sample_size(20);

    for samples in IntoIter::new([
        Samples::single(),
        Samples::grid(nonzero_ext::nonzero!(2u8)),
        Samples::grid(nonzero_ext::nonzero!(3u8)),
        Samples::grid(nonzero_ext::nonzero!(4u8)),
    ]) {
        group.bench_with_input(
            BenchmarkId::from_parameter(&samples),
            &samples,
            |b, samples| {
                let mut world = World::empty();
                world.lights.push(Light::point(
                    Colour::WHITE,
                    Point3D::new(-10.0, 10.0, -10.0),
                ));
                world.add(Object::sphere());

                let camera = Camera::new(
                    nonzero_ext::nonzero!(600u16),
                    nonzero_ext::nonzero!(600u16),
                    PI / 3.0,
                    Transform::view_transform(
                        Point3D::new(0.0, 1.0, -5.0),
                        Point3D::new(0.0, 0.0, 0.0),
                        Normal3D::POSITIVE_Y,
                    ),
                );

                b.iter(|| {
                    renderer::render(&world, &camera, samples);
                });
            },
        );
    }
}

fn basic_scene(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic scene with anti-aliasing");
    group.sample_size(10);

    for samples in IntoIter::new([
        Samples::single(),
        Samples::grid(nonzero_ext::nonzero!(2u8)),
        Samples::grid(nonzero_ext::nonzero!(3u8)),
        Samples::grid(nonzero_ext::nonzero!(4u8)),
    ]) {
        group.bench_with_input(
            BenchmarkId::from_parameter(&samples),
            &samples,
            |b, samples| {
                let mut world = World::empty();
                world
                    .lights
                    .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));
                world.add(Object::plane().with_material(Material {
                    kind: MaterialKind::Pattern(Pattern::checkers(Colour::WHITE, Colour::BLACK)),
                    ..Default::default()
                }));
                world.add(
                    Object::sphere()
                        .with_material(Material {
                            kind: MaterialKind::Solid(Colour::RED),
                            ..Default::default()
                        })
                        .transformed(Transform::identity().translate_y(1.0).translate_z(-2.0)),
                );
                world.add(
                    Object::plane()
                        .with_material(Material {
                            kind: MaterialKind::Solid(Colour::new(0.1, 0.1, 0.6)),
                            ..Default::default()
                        })
                        .transformed(
                            Transform::identity()
                                .rotate_x(-PI / 2.0)
                                .rotate_y(-PI / 3.0)
                                .translate_z(7.5),
                        ),
                );
                world.add(
                    Object::plane()
                        .with_material(Material {
                            kind: MaterialKind::Solid(Colour::BLACK),
                            reflective: 0.9,
                            ..Default::default()
                        })
                        .transformed(
                            Transform::identity()
                                .rotate_x(-PI / 2.0)
                                .rotate_y(PI / 5.0)
                                .translate_z(7.5),
                        ),
                );

                let camera = Camera::new(
                    nonzero_ext::nonzero!(400u16),
                    nonzero_ext::nonzero!(400u16),
                    1.2,
                    Transform::view_transform(
                        Point3D::new(0.0, 2.5, -10.0),
                        Point3D::new(0.0, 1.0, 0.0),
                        Normal3D::POSITIVE_Y,
                    ),
                );

                b.iter(|| {
                    renderer::render(&world, &camera, samples);
                });
            },
        );
    }
}
