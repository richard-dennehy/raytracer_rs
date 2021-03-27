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
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(-6.0, 15.0, -8.0)));

    {
        let wall = Object::plane()
            .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(15.0))
            .with_material(Material {
                pattern: Pattern::solid(Colour::WHITE),
                specular: 0.1,
                ..Default::default()
            });
        world.objects.push(wall);
    }

    {
        let floor = Object::plane().with_material(Material {
            pattern: Pattern::solid(Colour::WHITE),
            diffuse: 0.9,
            // have to crank the ambient up so it actually appears white rather than grey
            ambient: 0.35,
            ..Default::default()
        });

        world.objects.push(floor);
    }

    {
        let red_pane = Object::cube()
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_z(4.5)
                    .translate_y(3.0)
                    .translate_x(-3.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.33, 0.0, 0.0)),
                transparency: 0.9,
                ..Default::default()
            });

        world.objects.push(red_pane);
    }

    {
        let green_pane = Object::cube()
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_z(4.5)
                    .translate_y(3.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.0, 0.33, 0.0)),
                transparency: 0.9,
                ..Default::default()
            });

        world.objects.push(green_pane);
    }

    {
        let blue_pane = Object::cube()
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_z(4.5)
                    .translate_y(3.0)
                    .translate_x(3.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.0, 0.0, 0.33)),
                transparency: 0.9,
                ..Default::default()
            });

        world.objects.push(blue_pane);
    }

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Transform::view_transform(
            Point3D::new(0.0, 4.0, -3.0),
            Point3D::new(0.0, 3.5, 0.0),
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
