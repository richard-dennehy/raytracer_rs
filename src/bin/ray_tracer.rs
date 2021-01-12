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
        Point3D::new(-10.0, 10.0, -10.0),
    ));

    {
        let wall = Object::plane()
            .with_transform(Matrix4D::rotation_x(-PI / 2.0).with_translation(0.0, 0.0, 5.1))
            .with_material(Material {
                pattern: Pattern::checkers(Colour::BLACK, Colour::WHITE)
                    .with_transform(Matrix4D::translation(0.0, 0.0, 0.1)),
                ..Default::default()
            });

        world.objects.push(wall);
    };

    {
        let outer_glass_sphere = Object::sphere()
            .with_transform(Matrix4D::translation(0.0, 1.0, 0.5))
            .with_material(Material {
                pattern: Pattern::solid(Colour::BLACK),
                transparency: 1.0,
                refractive: 1.5,
                reflective: 1.0,
                ..Default::default()
            });

        world.objects.push(outer_glass_sphere);
    };

    {
        let inner_air_sphere = Object::sphere()
            .with_transform(Matrix4D::uniform_scaling(0.5).with_translation(0.0, 1.0, 0.5))
            .with_material(Material {
                pattern: Pattern::solid(Colour::BLACK),
                transparency: 1.0,
                refractive: 1.0,
                reflective: 1.0,
                ..Default::default()
            });

        world.objects.push(inner_air_sphere);
    };

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 1.5, -3.0),
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed())
}
