use criterion::{black_box, criterion_group, Criterion};
use ray_tracer::{
    Camera, Colour, Light, Material, Normal3D, Object, Pattern, Point3D, Transform, World,
};
use std::f64::consts::PI;

criterion_group! {
    benches,
    single_ray_many_objects,
    single_ray_many_reflective_refractive_objects
}

fn single_ray_many_objects(c: &mut Criterion) {
    c.bench_function("cast single ray into scene with lots of objects", |b| {
        let mut world = World::empty();

        let cube_size = 50;
        let spacing = 2.7;

        for x in 0..cube_size {
            for y in 0..cube_size {
                for z in 0..cube_size {
                    let x = x as f64;
                    let y = y as f64;
                    let z = z as f64;
                    let cube_size = cube_size as f64;

                    let colour = Colour::new(x / cube_size, y / cube_size, z / cube_size);

                    let sphere = Object::sphere()
                        .transformed(
                            Transform::identity()
                                .translate_z(z * spacing)
                                .translate_y(y * spacing)
                                .translate_x(x * spacing),
                        )
                        .with_material(Material {
                            pattern: Pattern::solid(colour),
                            ..Default::default()
                        });

                    world.objects.push(sphere);
                }
            }
        }

        let cube_size = cube_size as f64;
        let approx_centre = cube_size * spacing / 2.0;

        world.lights.push(Light::point(
            Colour::greyscale(0.95),
            Point3D::new(
                approx_centre * 2.8,
                approx_centre * 3.7,
                approx_centre * 3.7,
            ),
        ));
        world.lights.push(Light::point(
            Colour::greyscale(0.95),
            Point3D::new(
                approx_centre * -2.8,
                approx_centre * 3.7,
                approx_centre * -3.7,
            ),
        ));

        let camera = Camera::new(
            nonzero_ext::nonzero!(800u16),
            nonzero_ext::nonzero!(800u16),
            PI / 3.0,
            Transform::view_transform(
                Point3D::new(
                    -approx_centre * 2.2,
                    approx_centre * 2.4,
                    approx_centre * -3.2,
                ),
                Point3D::new(approx_centre, approx_centre - spacing, approx_centre),
                Normal3D::POSITIVE_Y,
            ),
        );

        b.iter(|| {
            black_box(world.colour_at(camera.ray_at(400, 400)));
        })
    });
}

fn single_ray_many_reflective_refractive_objects(c: &mut Criterion) {
    c.bench_function(
        "cast single ray into scene with lots of reflective, transparent, and refractive objects",
        |b| {
            let mut world = World::empty();

            let cube_size = 20;
            let spacing = 2.7;

            for x in 0..cube_size {
                for y in 0..cube_size {
                    for z in 0..cube_size {
                        let x = x as f64;
                        let y = y as f64;
                        let z = z as f64;
                        let cube_size = cube_size as f64;

                        let colour = Colour::new(x / cube_size, y / cube_size, z / cube_size);

                        let sphere = Object::sphere()
                            .transformed(
                                Transform::identity()
                                    .translate_z(z * spacing)
                                    .translate_y(y * spacing)
                                    .translate_x(x * spacing),
                            )
                            .with_material(Material {
                                pattern: Pattern::solid(colour),
                                reflective: 0.5,
                                transparency: 0.5,
                                refractive: 1.2,
                                ..Default::default()
                            });

                        world.objects.push(sphere);
                    }
                }
            }

            let cube_size = cube_size as f64;
            let approx_centre = cube_size * spacing / 2.0;

            world.lights.push(Light::point(
                Colour::greyscale(0.95),
                Point3D::new(
                    approx_centre * 2.8,
                    approx_centre * 3.7,
                    approx_centre * 3.7,
                ),
            ));
            world.lights.push(Light::point(
                Colour::greyscale(0.95),
                Point3D::new(
                    approx_centre * -2.8,
                    approx_centre * 3.7,
                    approx_centre * -3.7,
                ),
            ));

            let camera = Camera::new(
                nonzero_ext::nonzero!(800u16),
                nonzero_ext::nonzero!(800u16),
                PI / 3.0,
                Transform::view_transform(
                    Point3D::new(
                        -approx_centre * 2.2,
                        approx_centre * 2.4,
                        approx_centre * -3.2,
                    ),
                    Point3D::new(approx_centre, approx_centre - spacing, approx_centre),
                    Normal3D::POSITIVE_Y,
                ),
            );

            b.iter(|| {
                black_box(world.colour_at(camera.ray_at(400, 400)));
            })
        },
    );
}
