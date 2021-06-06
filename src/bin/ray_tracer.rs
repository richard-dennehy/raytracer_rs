extern crate ray_tracer;

use rand::Rng;
use ray_tracer::renderer::Samples;
use ray_tracer::*;
use std::f64::consts::FRAC_PI_4;
use std::time::Instant;

/// Notes on axes and rotation:
/// X axis runs from left (negative values) to right (positive values) of default camera view
/// Y axis runs from bottom (negative values) to top (positive values) of default camera view
/// Z axis runs from behind/closer (negative values) to in front/away (positive values) of default camera view i.e. larger +Z values move objects away from the camera; smaller +Z values keep objects close
/// Rotation in positive X rotates the far side (+Z) down (-Y), and the near side (-Z) up (+Y), therefore rotation in X should normally be negative
/// Rotation in positive Y moves the right side (+X) closer to the camera (-Z) and the left side (-X) further from the camera (+Z)

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let seed = rand::thread_rng().gen::<u64>();
    dbg!(seed);

    let mut world = World::empty();
    world.lights.push(Light::area(
        Colour::greyscale(1.5),
        Point3D::new(-1.0, 2.0, 4.0),
        Vector3D::new(2.0, 0.0, 0.0),
        Vector3D::new(0.0, 2.0, 0.0),
        nonzero_ext::nonzero!(15u8),
        nonzero_ext::nonzero!(15u8),
        seed,
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

    let red_sphere = Object::sphere()
        .with_material(Material {
            kind: MaterialKind::Solid(Colour::RED),
            ambient: 0.1,
            specular: 0.0,
            diffuse: 0.6,
            reflective: 0.3,
            ..Default::default()
        })
        .transformed(
            Transform::identity()
                .scale_all(0.5)
                .translate_x(0.5)
                .translate_y(0.5),
        );
    world.add(red_sphere);

    let blue_sphere = Object::sphere()
        .with_material(Material {
            kind: MaterialKind::Solid(Colour::new(0.5, 0.5, 1.0)),
            ambient: 0.1,
            specular: 0.0,
            diffuse: 0.6,
            reflective: 0.3,
            ..Default::default()
        })
        .transformed(
            Transform::identity()
                .scale_all(1.0 / 3.0)
                .translate_x(-0.25)
                .translate_y(1.0 / 3.0),
        );
    world.add(blue_sphere);

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_4,
        Transform::view_transform(
            Point3D::new(-3.0, 1.0, 2.5),
            Point3D::new(0.0, 0.5, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(world, camera, &Samples::single());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
