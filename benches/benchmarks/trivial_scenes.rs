use criterion::{criterion_group, BenchmarkId, Criterion};
use ray_tracer::{
    renderer, Camera, Colour, Light, Matrix4D, Object, Point3D, Ray, Vector3D, World,
};
use std::f64::consts::PI;
use std::num::NonZeroU16;

criterion_group! {
    benches,
    single_sphere_single_ray,
    single_sphere_full_render,
    single_plane_full_render,
    single_cube_full_render,
    single_cylinder_full_render,
    single_cone_full_render,
}

fn single_sphere_single_ray(c: &mut Criterion) {
    let world = single_sphere_world();

    c.bench_function("cast single ray at single sphere", |b| {
        b.iter(|| {
            world.colour_at(Ray::new(
                Point3D::new(0.0, 0.0, -10.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ))
        })
    });
}

fn single_sphere_full_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render full scene with single sphere");
    group.sample_size(40);

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let world = single_sphere_world();

                    let camera = Camera::new(
                        NonZeroU16::new(*x).unwrap(),
                        NonZeroU16::new(*y).unwrap(),
                        PI / 3.0,
                        Matrix4D::view_transform(
                            Point3D::new(0.0, 0.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Vector3D::new(0.0, 1.0, 0.0),
                        ),
                    );

                    renderer::render(world, camera);
                })
            },
        );
    }
}

fn single_plane_full_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render full scene with single plane");
    group.sample_size(40);

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut world = single_light_world();
                    world.objects.push(Object::plane());

                    let camera = Camera::new(
                        NonZeroU16::new(*x).unwrap(),
                        NonZeroU16::new(*y).unwrap(),
                        PI / 3.0,
                        Matrix4D::view_transform(
                            Point3D::new(0.0, 0.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Vector3D::new(0.0, 1.0, 0.0),
                        ),
                    );

                    renderer::render(world, camera);
                })
            },
        );
    }
}

fn single_cube_full_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render full scene with single cube");
    group.sample_size(40);

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut world = single_light_world();
                    world.objects.push(Object::cube());

                    let camera = Camera::new(
                        NonZeroU16::new(*x).unwrap(),
                        NonZeroU16::new(*y).unwrap(),
                        PI / 3.0,
                        Matrix4D::view_transform(
                            Point3D::new(0.0, 0.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Vector3D::new(0.0, 1.0, 0.0),
                        ),
                    );

                    renderer::render(world, camera);
                })
            },
        );
    }
}

fn single_cylinder_full_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render full scene with single cylinder");
    group.sample_size(40);

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut world = single_light_world();
                    world.objects.push(Object::cylinder().build());

                    let camera = Camera::new(
                        NonZeroU16::new(*x).unwrap(),
                        NonZeroU16::new(*y).unwrap(),
                        PI / 3.0,
                        Matrix4D::view_transform(
                            Point3D::new(0.0, 0.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Vector3D::new(0.0, 1.0, 0.0),
                        ),
                    );

                    renderer::render(world, camera);
                })
            },
        );
    }
}

fn single_cone_full_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render full scene with single cone");
    group.sample_size(40);

    for (x, y) in RESOLUTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}x{:?}", x, y)),
            &(*x, *y),
            |b, (x, y)| {
                b.iter(|| {
                    let mut world = single_light_world();
                    world.objects.push(Object::cone().build());

                    let camera = Camera::new(
                        NonZeroU16::new(*x).unwrap(),
                        NonZeroU16::new(*y).unwrap(),
                        PI / 3.0,
                        Matrix4D::view_transform(
                            Point3D::new(0.0, 0.0, -5.0),
                            Point3D::new(0.0, 0.0, 0.0),
                            Vector3D::new(0.0, 1.0, 0.0),
                        ),
                    );

                    renderer::render(world, camera);
                })
            },
        );
    }
}

#[inline(always)]
fn single_sphere_world() -> World {
    let mut world = single_light_world();

    world
        .objects
        .push(Object::sphere().with_transform(Matrix4D::translation(0.0, 0.0, 0.5)));

    world
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

const RESOLUTIONS: [(u16, u16); 3] = [(400, 400), (800, 800), (1920, 1080)];
