extern crate ray_tracer;

use ray_tracer::*;
use std::f64::consts::PI;
use std::num::NonZeroU16;
use std::time::Instant;

#[macro_use]
extern crate nonzero_ext;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(600u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(600u16);

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (-Z) up (+Y), and the near side (+Z) down (-Y)
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world.lights.push(Light::point(
        Colour::WHITE,
        Point3D::new(-10.0, 10.0, -10.0),
    ));

    {
        let wall = Object::plane()
            .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(5.1))
            .with_material(Material {
                pattern: Pattern::checkers(Colour::BLACK, Colour::WHITE),
                ..Default::default()
            });

        world.objects.push(wall);
    };

    {
        let outer_glass_sphere = Object::sphere()
            .transformed(Transform::identity().translate_y(1.0).translate_z(0.5))
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
            .transformed(
                Transform::identity()
                    .scale_all(0.5)
                    .translate_y(1.0)
                    .translate_z(0.5),
            )
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
        Transform::view_transform(
            Point3D::new(0.0, 1.5, -3.0),
            Point3D::new(0.0, 1.0, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(world, camera);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
