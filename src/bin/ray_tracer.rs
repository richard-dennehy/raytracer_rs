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

    let mut floor = Object::plane();
    floor.material.pattern = Pattern::striped(Colour::new(1.0, 0.9, 0.9), Colour::WHITE);
    floor.material.specular = 0.0;

    let mut middle_sphere = Object::sphere().with_transform(Matrix4D::translation(-0.5, 1.0, 0.5));
    middle_sphere.material.pattern = Pattern::striped(Colour::new(0.1, 1.0, 0.5), Colour::BLACK);
    middle_sphere.material.diffuse = 0.7;
    middle_sphere.material.specular = 0.3;

    let mut right_sphere = Object::sphere()
        .with_transform(Matrix4D::uniform_scaling(0.5).with_translation(1.5, 0.5, -0.5));
    right_sphere.material.pattern = Pattern::striped(Colour::new(0.1, 1.0, 0.5), Colour::WHITE);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;

    let mut left_sphere = Object::sphere()
        .with_transform(Matrix4D::uniform_scaling(0.33).with_translation(-1.5, 0.33, -0.75));
    left_sphere.material.pattern = Pattern::striped(Colour::new(1.0, 0.8, 0.1), Colour::BLACK);
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.3;

    let mut world = World::empty();
    world.lights.push(PointLight::new(
        Colour::WHITE,
        Point3D::new(-10.0, 10.0, -10.0),
    ));
    world.objects.push(floor);
    world.objects.push(middle_sphere);
    world.objects.push(right_sphere);
    world.objects.push(left_sphere);

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 1.5, -5.0),
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed())
}
