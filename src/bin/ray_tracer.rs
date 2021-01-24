extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::f64::consts::PI;
use std::num::NonZeroU16;
use std::time::Instant;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(1920u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(1080u16);

fn main() {
    let timer = Instant::now();

    let mut world = World::empty();
    world.lights.push(PointLight::new(
        Colour::WHITE,
        Point3D::new(-10.0, 12.0, -10.0),
    ));

    fn hexagon_corner() -> Object {
        Object::sphere()
            .with_transform(Matrix4D::uniform_scaling(0.25).with_translation(0.0, 0.0, -1.0))
    }

    fn hexagon_edge() -> Object {
        Object::cylinder()
            .min_y(0.0)
            .max_y(1.0)
            .build()
            .with_transform(
                Matrix4D::scaling(0.25, 1.0, 0.25)
                    .with_rotation_z(-PI / 2.0)
                    .with_rotation_y(-PI / 6.0)
                    .with_translation(0.0, 0.0, -1.0),
            )
    }

    fn hexagon_side() -> Object {
        Object::group(vec![hexagon_corner(), hexagon_edge()])
    }

    let hexagon_parts = (0..6)
        .into_iter()
        .map(|i| hexagon_side().with_transform(Matrix4D::rotation_y(i as f64 * PI / 3.0)))
        .collect();
    let hexagon = Object::group(hexagon_parts);

    world.objects.push(hexagon);

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 2.5, -6.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    println!("Rendered in {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed())
}
