use criterion::{criterion_group, Criterion};
use ray_tracer::core::*;
use ray_tracer::renderer::{render, Camera, Samples};
use ray_tracer::scene::{Light, Material, MaterialKind, Object, World};
use std::f64::consts::FRAC_PI_4;

criterion_group! {
    benches,
    lighting_a_single_object_with_an_area_light,
    lighting_multiple_objects_with_an_area_light,
}

fn lighting_a_single_object_with_an_area_light(c: &mut Criterion) {
    let mut group = c.benchmark_group("lighting a scene with a single object using an area light");
    group.sample_size(20);

    group.bench_function("(4x4 samples)", |b| {
        let mut world = World::empty();
        world.lights.push(Light::area(
            Colour::WHITE,
            Point3D::new(-0.5, -0.5, -5.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            nonzero_ext::nonzero!(4u8),
            nonzero_ext::nonzero!(4u8),
            0,
        ));
        world.add(Object::sphere().with_material(Material {
            kind: MaterialKind::Solid(Colour::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.0,
            ..Default::default()
        }));

        let camera = Camera::new(
            nonzero_ext::nonzero!(400u16),
            nonzero_ext::nonzero!(400u16),
            FRAC_PI_4,
            Transform::view_transform(
                Point3D::new(0.0, 0.0, -5.0),
                Point3D::new(0.0, 0.0, -1.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        b.iter(|| render(&world, &camera, &Samples::single(), false));
    });
}

fn lighting_multiple_objects_with_an_area_light(c: &mut Criterion) {
    let mut group = c.benchmark_group("lighting a scene with multiple objects using an area light");
    group.sample_size(20);

    group.bench_function("(4x4 samples)", |b| {
        let mut world = World::empty();
        world.lights.push(Light::area(
            Colour::greyscale(1.5),
            Point3D::new(-1.0, 2.0, 4.0),
            Vector3D::new(2.0, 0.0, 0.0),
            Vector3D::new(0.0, 2.0, 0.0),
            nonzero_ext::nonzero!(4u8),
            nonzero_ext::nonzero!(4u8),
            7859535925052243674,
        ));

        let light_source = Object::cube()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::greyscale(1.5)),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                casts_shadow: false,
                ..Default::default()
            })
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_y(3.0)
                    .translate_z(4.0),
            );

        world.add(light_source);

        let floor = Object::plane().with_material(Material {
            kind: MaterialKind::Solid(Colour::WHITE),
            ambient: 0.025,
            diffuse: 0.67,
            specular: 0.0,
            ..Default::default()
        });

        world.add(floor);

        fn sphere_material(colour: Colour) -> Material {
            Material {
                kind: MaterialKind::Solid(colour),
                ambient: 0.1,
                specular: 0.0,
                diffuse: 0.6,
                reflective: 0.3,
                ..Default::default()
            }
        }

        let red_sphere = Object::sphere()
            .with_material(sphere_material(Colour::RED))
            .transformed(
                Transform::identity()
                    .scale_all(0.5)
                    .translate_x(0.5)
                    .translate_y(0.5),
            );
        world.add(red_sphere);

        let blue_sphere = Object::sphere()
            .with_material(sphere_material(Colour::new(0.5, 0.5, 1.0)))
            .transformed(
                Transform::identity()
                    .scale_all(1.0 / 3.0)
                    .translate_x(-0.25)
                    .translate_y(1.0 / 3.0),
            );
        world.add(blue_sphere);

        let camera = Camera::new(
            nonzero_ext::nonzero!(400u16),
            nonzero_ext::nonzero!(400u16),
            FRAC_PI_4,
            Transform::view_transform(
                Point3D::new(-3.0, 1.0, 2.5),
                Point3D::new(0.0, 0.5, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        b.iter(|| render(&world, &camera, &Samples::single(), false));
    });
}
