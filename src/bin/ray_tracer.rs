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
        let mut floor = Object::plane();
        floor.material.pattern =
            Pattern::striped(Colour::new(0.2, 0.2, 0.2), Colour::new(0.1, 0.1, 0.1))
                .with_transform(Matrix4D::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0));
        floor.material.reflective = 0.7;

        world.objects.push(floor);
    };

    {
        let right_wall = Object::plane()
            .with_transform(
                Matrix4D::rotation_x(-PI / 2.0)
                    .with_rotation_y(PI / 4.0)
                    .with_translation(0.0, 0.0, 5.0),
            )
            .with_material(Material {
                reflective: 0.7,
                pattern: Pattern::solid(Colour::new(0.15, 0.15, 0.15)),
                ..Default::default()
            });

        world.objects.push(right_wall);
    };

    {
        let left_wall = Object::plane()
            .with_transform(
                Matrix4D::rotation_x(-PI / 2.0)
                    .with_rotation_y(-PI / 4.0)
                    .with_translation(0.0, 0.0, 5.0),
            )
            .with_material(Material {
                reflective: 0.7,
                pattern: Pattern::solid(Colour::new(0.1, 0.1, 0.1)),
                ..Default::default()
            });

        world.objects.push(left_wall);
    };

    {
        let mut middle_sphere =
            Object::sphere().with_transform(Matrix4D::translation(-0.5, 1.0, 0.5));
        middle_sphere.material.pattern = Pattern::striped(Colour::BLUE, Colour::GREEN)
            .with_transform(Matrix4D::uniform_scaling(0.2).with_rotation_z(PI / 2.0));
        middle_sphere.material.diffuse = 0.7;
        middle_sphere.material.specular = 0.3;

        world.objects.push(middle_sphere);
    };

    {
        let mut right_sphere = Object::sphere()
            .with_transform(Matrix4D::uniform_scaling(0.5).with_translation(1.5, 0.5, -0.5));
        right_sphere.material.pattern = Pattern::gradient(Colour::new(1.0, 0.5, 0.0), Colour::RED);
        right_sphere.material.diffuse = 0.7;
        right_sphere.material.specular = 0.3;

        world.objects.push(right_sphere);
    };

    {
        let mut left_sphere = Object::sphere()
            .with_transform(Matrix4D::uniform_scaling(0.33).with_translation(-1.5, 0.33, -0.75));
        left_sphere.material.pattern = Pattern::ring(Colour::RED, Colour::WHITE)
            .with_transform(Matrix4D::uniform_scaling(0.33).with_rotation_x(PI / 2.0));
        left_sphere.material.diffuse = 0.7;
        left_sphere.material.specular = 0.3;

        world.objects.push(left_sphere);
    };

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
