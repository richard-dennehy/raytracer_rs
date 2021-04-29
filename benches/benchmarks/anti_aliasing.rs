use criterion::{criterion_group, BenchmarkId, Criterion};
use ray_tracer::renderer::Subsamples;
use ray_tracer::{
    renderer, Camera, Colour, Light, Material, Normal3D, Object, Pattern, Point3D, Transform, World,
};
use std::array::IntoIter;
use std::f64::consts::PI;

criterion_group! {
    benches,
    single_sphere,
    basic_scene
}

fn single_sphere(c: &mut Criterion) {
    let mut group = c.benchmark_group("single sphere with anti-aliasing");
    group.sample_size(20);

    for samples in IntoIter::new([
        Subsamples::None,
        Subsamples::X4,
        Subsamples::X8,
        Subsamples::X16,
    ]) {
        group.bench_with_input(
            BenchmarkId::from_parameter(samples),
            &samples,
            |b, samples| {
                b.iter(|| {
                    let mut world = World::empty();
                    world.lights.push(Light::point(
                        Colour::WHITE,
                        Point3D::new(-10.0, 10.0, -10.0),
                    ));
                    world.add(Object::sphere());

                    let camera = Camera::new(
                        nonzero_ext::nonzero!(800u16),
                        nonzero_ext::nonzero!(800u16),
                        PI / 3.0,
                        Transform::view_transform(
                            Point3D::new(0.0, 1.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Normal3D::POSITIVE_Y,
                        ),
                    );

                    renderer::render(world, camera, *samples);
                });
            },
        );
    }
}

fn basic_scene(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic scene with anti-aliasing");
    group.sample_size(10);

    for samples in IntoIter::new([
        Subsamples::None,
        Subsamples::X4,
        Subsamples::X8,
        Subsamples::X16,
    ]) {
        group.bench_with_input(
            BenchmarkId::from_parameter(samples),
            &samples,
            |b, samples| {
                b.iter(|| {
                    let mut world = World::empty();
                    world
                        .lights
                        .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));
                    world.add(Object::plane().with_material(Material {
                        pattern: Pattern::checkers(Colour::WHITE, Colour::BLACK),
                        ..Default::default()
                    }));
                    world.add(
                        Object::sphere()
                            .with_material(Material {
                                pattern: Pattern::solid(Colour::RED),
                                ..Default::default()
                            })
                            .transformed(Transform::identity().translate_y(1.0).translate_z(-2.0)),
                    );
                    world.add(
                        Object::plane()
                            .with_material(Material {
                                pattern: Pattern::solid(Colour::new(0.1, 0.1, 0.6)),
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
                                pattern: Pattern::solid(Colour::BLACK),
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
                        nonzero_ext::nonzero!(800u16),
                        nonzero_ext::nonzero!(800u16),
                        1.2,
                        Transform::view_transform(
                            Point3D::new(0.0, 2.5, -10.0),
                            Point3D::new(0.0, 1.0, 0.0),
                            Normal3D::POSITIVE_Y,
                        ),
                    );

                    renderer::render(world, camera, *samples);
                });
            },
        );
    }
}
