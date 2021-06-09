use criterion::{criterion_group, Criterion};
use nonzero_ext::*;
use ray_tracer::renderer::Samples;
use ray_tracer::{
    renderer, yaml_parser, Camera, Colour, Light, Material, MaterialKind, Normal3D, Object,
    Pattern, Point3D, Transform, World,
};
use std::f64::consts::PI;
use std::fs;
use std::path::Path;

criterion_group! {
    benches,
    cover_image,
    reflect_refract,
    fresnel,
}

fn cover_image(c: &mut Criterion) {
    let mut group = c.benchmark_group("render cover image from YAML");
    group.sample_size(10);

    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scene_descriptions/cover.yml"),
    )
    .unwrap();

    group.bench_function("600x600", |b| {
        b.iter(|| {
            let mut scene = yaml_parser::parse(&yaml).unwrap();
            scene.override_resolution(600, 600);

            let mut world = World::empty();
            world.lights.append(&mut scene.lights());
            world.add(Object::group(scene.objects().unwrap()));

            let camera = scene.camera().unwrap();

            renderer::render(world, camera, &Samples::single());
        })
    });
}

fn reflect_refract(c: &mut Criterion) {
    let mut group = c.benchmark_group("render reflection + refraction image from YAML");
    group.sample_size(10);

    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scene_descriptions/reflect-refract.yml"),
    )
    .unwrap();

    group.bench_function("600x600", |b| {
        b.iter(|| {
            let mut scene = yaml_parser::parse(&yaml).unwrap();
            scene.override_resolution(600, 600);

            let mut world = World::empty();
            world.lights.append(&mut scene.lights());
            world.add(Object::group(scene.objects().unwrap()));

            let camera = scene.camera().unwrap();

            renderer::render(world, camera, &Samples::single());
        })
    });
}

fn fresnel(c: &mut Criterion) {
    let mut group = c.benchmark_group("fresnel effect in transparent reflective material");
    group.sample_size(10);

    group.bench_function("600x600", |b| {
        b.iter(|| {
            let mut world = World::empty();
            world.lights.push(Light::point(
                Colour::WHITE,
                Point3D::new(-10.0, 10.0, -10.0),
            ));

            {
                let wall = Object::plane()
                    .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(5.1))
                    .with_material(Material {
                        kind: MaterialKind::Pattern(Pattern::checkers(
                            Colour::BLACK,
                            Colour::WHITE,
                        )),
                        ..Default::default()
                    });

                world.add(wall);
            };

            {
                let outer_glass_sphere = Object::sphere()
                    .transformed(Transform::identity().translate_y(1.0).translate_z(0.5))
                    .with_material(Material {
                        kind: MaterialKind::Pattern(Pattern::solid(Colour::BLACK)),
                        transparency: 1.0,
                        refractive: 1.5,
                        reflective: 1.0,
                        ..Default::default()
                    });

                world.add(outer_glass_sphere);
            };

            {
                let inner_air_sphere = Object::sphere()
                    .transformed(
                        Transform::identity()
                            .scale_all(0.5)
                            .translate_y(1.0)
                            .translate_z(0.5),
                    )
                    .with_material(Material {
                        kind: MaterialKind::Pattern(Pattern::solid(Colour::BLACK)),
                        transparency: 1.0,
                        refractive: 1.0,
                        reflective: 1.0,
                        ..Default::default()
                    });

                world.add(inner_air_sphere);
            };

            let camera = Camera::new(
                nonzero!(600u16),
                nonzero!(600u16),
                PI / 3.0,
                Transform::view_transform(
                    Point3D::new(0.0, 1.5, -3.0),
                    Point3D::new(0.0, 1.0, 0.0),
                    Normal3D::POSITIVE_Y,
                ),
            );

            renderer::render(world, camera, &Samples::single());
        })
    });
}
