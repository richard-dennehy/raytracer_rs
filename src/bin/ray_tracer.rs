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

    {
        let cone =
            Object::double_napped_cone().with_transform(Matrix4D::translation(0.0, 0.0, 5.0));
        world.objects.push(cone);
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

    println!("Rendered in {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed())
}
