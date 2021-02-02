extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::convert::TryInto;
use std::f64::consts::PI;
use std::fs;
use std::num::NonZeroU16;
use std::time::Instant;

const CAMERA_WIDTH: NonZeroU16 = nonzero!(400u16);
const CAMERA_HEIGHT: NonZeroU16 = nonzero!(300u16);

fn main() -> Result<(), String> {
    let timer = Instant::now();

    let mut world = World::empty();
    world.lights.push(PointLight::new(
        Colour::WHITE,
        Point3D::new(-10.0, 12.0, -10.0),
    ));

    let wavefront_file = fs::read_to_string("susan.obj").map_err(|e| e.to_string())?;
    let susan: Object = obj_parser::parse(&wavefront_file).try_into()?;
    let susan = susan.with_transform(
        Matrix4D::rotation_x(PI / 2.0)
            .with_rotation_z(PI)
            .with_rotation_x(-PI / 2.0),
    );

    world.objects.push(susan);

    let camera = Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        PI / 3.0,
        Matrix4D::view_transform(
            Point3D::new(0.0, 0.0, -5.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let canvas = renderer::render(world, camera);

    println!("Rendered in {:.2?}", timer.elapsed());

    let image = image_writer::write(canvas);
    image.save("out.png").expect("failed to write output file");

    println!("Completed in {:.2?}", timer.elapsed());

    Ok(())
}
