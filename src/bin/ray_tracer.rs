extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::f64::consts::PI;
use std::fs;
use std::num::NonZeroU16;
use std::time::Instant;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(800u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(600u16);

fn main() {
    let timer = Instant::now();

    let mut world = World::empty();
    world.lights.push(PointLight::new(
        Colour::WHITE,
        Point3D::new(-10.0, 12.0, -10.0),
    ));

    {
        let floor = Object::plane().with_material(Material {
            pattern: Pattern::checkers(Colour::WHITE, Colour::BLACK),
            reflective: 0.2,
            ..Default::default()
        });

        world.objects.push(floor);
    };

    {
        let mirror = Object::plane()
            .with_transform(
                Matrix4D::rotation_x(-PI / 2.0)
                    .with_rotation_y(PI / 4.0)
                    .with_translation(0.0, 0.0, 5.0),
            )
            .with_material(Material {
                reflective: 1.0,
                pattern: Pattern::solid(Colour::new(0.05, 0.05, 0.05)),
                ..Default::default()
            });

        world.objects.push(mirror);
    };

    {
        let wall = Object::plane()
            .with_transform(
                Matrix4D::rotation_x(-PI / 2.0)
                    .with_rotation_y(-PI / 4.0)
                    .with_translation(-1.0, 0.0, 5.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.98, 0.98, 0.98)),
                ..Default::default()
            });

        world.objects.push(wall);
    };

    {
        let cube = Object::cube()
            .with_transform(Matrix4D::translation(-1.5, 1.0, 0.0))
            .with_material(Material {
                pattern: Pattern::checkers(Colour::BLUE, Colour::RED)
                    .with_transform(Matrix4D::uniform_scaling(0.33)),
                ..Default::default()
            });

        world.objects.push(cube);
    };

    {
        let cylinder = Object::capped_cylinder(0.0, 2.0)
            .with_transform(Matrix4D::translation(1.0, 0.0, 1.0))
            .with_material(Material {
                pattern: Pattern::solid(Colour::GREEN),
                ..Default::default()
            });

        world.objects.push(cylinder);
    };

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 2.5, -6.0),
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed())
}
