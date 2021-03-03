use criterion::{criterion_group, BenchmarkId, Criterion};
use nonzero_ext::*;
use ray_tracer::{
    renderer, Camera, Colour, Light, Object, Point3D, Ray, Transform, Vector3D, World,
};
use std::f64::consts::PI;

criterion_group! {
    benches,
    empty_scene_full_render,
    single_sphere_single_ray,
    single_object_full_render,
}

fn single_sphere_single_ray(c: &mut Criterion) {
    let mut world = single_light_world();
    world.objects.push(Object::sphere());

    c.bench_function("cast single ray at single sphere", |b| {
        b.iter(|| {
            world.colour_at(Ray::new(
                Point3D::new(0.0, 0.0, -10.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ))
        })
    });
}

// test loop overhead
fn empty_scene_full_render(c: &mut Criterion) {
    c.bench_function("render empty scene at 1920x1080", |b| {
        b.iter(|| {
            let world = World::empty();

            let camera = Camera::new(
                nonzero!(1920u16),
                nonzero!(1080u16),
                PI / 3.0,
                Transform::view_transform(
                    Point3D::new(0.0, 0.0, -5.0),
                    Point3D::new(0.0, 0.0, 0.0),
                    Vector3D::new(0.0, 1.0, 0.0),
                ),
            );

            renderer::render(world, camera);
        })
    });
}

// compare primitives/actually render stuff
fn single_object_full_render(c: &mut Criterion) {
    // awkward way to dynamically create primitives for tests
    // - need explicit type or type inference gets very upset
    // - need `fn` because these aren't `Copy` (and can't be)
    let shapes: Vec<(&str, Box<fn() -> Object>)> = vec![
        ("sphere", Box::new(|| Object::sphere())),
        ("plane", Box::new(|| Object::plane())),
        ("cube", Box::new(|| Object::cube())),
        ("cylinder", Box::new(|| Object::cylinder().build())),
        ("cone", Box::new(|| Object::cone().build())),
    ];

    let mut group = c.benchmark_group("render scene with single object (1920x1080)");
    group.sample_size(50);

    for (name, shape) in shapes.into_iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), &shape, |b, shape| {
            b.iter(|| {
                let mut world = single_light_world();
                world.objects.push(shape());

                let camera = Camera::new(
                    nonzero!(1920u16),
                    nonzero!(1080u16),
                    PI / 3.0,
                    Transform::view_transform(
                        Point3D::new(0.0, 0.0, -5.0),
                        Point3D::new(0.0, 0.0, 0.0),
                        Vector3D::new(0.0, 1.0, 0.0),
                    ),
                );

                renderer::render(world, camera);
            })
        });
    }
}

#[inline(always)]
fn single_light_world() -> World {
    let mut world = World::empty();
    world.lights.push(Light::point(
        Colour::WHITE,
        Point3D::new(-10.0, 10.0, -10.0),
    ));

    world
}
