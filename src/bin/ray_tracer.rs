extern crate ray_tracer;

use ray_tracer::*;
use std::f64::consts::PI;
use std::num::NonZeroU16;
use std::time::Instant;

#[macro_use]
extern crate nonzero_ext;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(1920u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(1080u16);

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (-Z) down (+Y), and the near side (+Z) up (-Y), therefore rotation in X should normally be negative
/// Rotation in positive Y moves the right side (+X) closer to the camera and the left side (-X) further from the camera

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world.settings.sky_colour = Colour::greyscale(0.9);

    let cube_size = 4;
    let spacing = 2.7;

    let mut spheres = Vec::with_capacity((cube_size as usize).pow(3));
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

                spheres.push(sphere);
            }
        }
    }

    world.objects.push(Object::group(spheres));

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
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
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

    let canvas = renderer::render(world, camera);

    println!("Rendered at {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
