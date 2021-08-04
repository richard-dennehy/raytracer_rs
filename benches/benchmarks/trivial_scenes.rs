use criterion::{criterion_group, BenchmarkId, Criterion};
use nonzero_ext::*;
use ray_tracer::core::*;
use ray_tracer::renderer;
use ray_tracer::renderer::{Camera, Samples};
use ray_tracer::scene::{Light, Material, MaterialKind, Object, UvPattern, World};
use std::f64::consts::PI;

criterion_group! {
    benches,
    empty_scene_full_render,
    single_sphere_single_ray,
    single_object_full_render,
}

fn single_sphere_single_ray(c: &mut Criterion) {
    let mut world = single_light_world();
    world.add(Object::sphere());

    c.bench_function("cast single ray at single sphere", |b| {
        b.iter(|| {
            world.colour_at(Ray::new(
                Point3D::new(0.0, 0.0, -10.0),
                Normal3D::POSITIVE_Z,
            ))
        })
    });
}

// test loop overhead
fn empty_scene_full_render(c: &mut Criterion) {
    c.bench_function("render empty scene at 1920x1080", |b| {
        let world = World::empty();

        let camera = Camera::new(
            nonzero!(1920u16),
            nonzero!(1080u16),
            PI / 3.0,
            Transform::view_transform(
                Point3D::new(0.0, 0.0, -5.0),
                Point3D::new(0.0, 0.0, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        b.iter(|| {
            renderer::render(&world, &camera, &Samples::single(), false);
        })
    });
}

// compare primitives/actually render stuff
fn single_object_full_render(c: &mut Criterion) {
    fn uv_checkers() -> UvPattern {
        UvPattern::checkers(
            Colour::BLACK,
            Colour::WHITE,
            nonzero_ext::nonzero!(1usize),
            nonzero_ext::nonzero!(1usize),
        )
    }
    fn uv_material() -> Material {
        Material {
            kind: MaterialKind::Uv(uv_checkers()),
            ..Default::default()
        }
    }
    // awkward way to dynamically create primitives for tests
    // - need explicit type or type inference gets very upset
    // - need `fn` because these aren't `Copy` (and can't be)
    let shapes: Vec<(&str, Box<fn() -> Object>)> = vec![
        ("sphere", Box::new(|| Object::sphere())),
        (
            "sphere (UV)",
            Box::new(|| Object::sphere().with_material(uv_material())),
        ),
        ("plane", Box::new(|| Object::plane())),
        (
            "plane (UV)",
            Box::new(|| Object::plane().with_material(uv_material())),
        ),
        ("cube", Box::new(|| Object::cube())),
        (
            "cube (UV)",
            Box::new(|| {
                Object::cube().with_material(Material {
                    kind: MaterialKind::Uv(UvPattern::cubic(
                        uv_checkers(),
                        uv_checkers(),
                        uv_checkers(),
                        uv_checkers(),
                        uv_checkers(),
                        uv_checkers(),
                    )),
                    ..Default::default()
                })
            }),
        ),
        ("cylinder", Box::new(|| Object::cylinder().build())),
        (
            "cylinder (UV, uncapped)",
            Box::new(|| Object::cylinder().build().with_material(uv_material())),
        ),
        ("cone", Box::new(|| Object::cone().build())),
        (
            "cone (UV, uncapped)",
            Box::new(|| Object::cone().build().with_material(uv_material())),
        ),
        (
            "triangle",
            Box::new(|| {
                Object::triangle(
                    Point3D::ORIGIN,
                    Point3D::new(0.0, 1.0, 0.0),
                    Point3D::new(1.0, 0.0, 0.0),
                )
            }),
        ),
        (
            "triangle (UV)",
            Box::new(|| {
                Object::triangle(
                    Point3D::ORIGIN,
                    Point3D::new(0.0, 1.0, 0.0),
                    Point3D::new(1.0, 0.0, 0.0),
                )
                .with_material(uv_material())
            }),
        ),
    ];

    let mut group = c.benchmark_group("render scene with single object (1920x1080)");
    group.sample_size(50);

    for (name, shape) in shapes.into_iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), &shape, |b, shape| {
            let mut world = single_light_world();
            world.add(shape());

            let camera = Camera::new(
                nonzero!(1920u16),
                nonzero!(1080u16),
                PI / 3.0,
                Transform::view_transform(
                    Point3D::new(0.0, 1.0, -5.0),
                    Point3D::new(0.0, 0.0, 0.0),
                    Normal3D::POSITIVE_Y,
                ),
            );

            b.iter(|| {
                renderer::render(&world, &camera, &Samples::single(), false);
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
