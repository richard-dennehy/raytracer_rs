extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::fs;
use std::num::NonZeroU16;

const WIDTH: NonZeroU16 = nonzero!(800u16);
const HEIGHT: NonZeroU16 = nonzero!(800u16);

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();
    let mut sphere = Sphere::unit();
    sphere.transform(Matrix4D::scaling(50.0, 50.0, 1.0).with_translation(
        (WIDTH.get() / 2) as _,
        (HEIGHT.get() / 2) as _,
        1.0,
    ));

    let direction = Vector3D::new(0.0, 0.0, 1.0);

    for x in 0..WIDTH.get() {
        for y in 0..HEIGHT.get() {
            let ray = Ray::new(Point3D::new(x as _, y as _, 0.0), direction);
            let intersection = ray.intersect(&sphere);
            if let Some(intersection) = intersection {
                let hit = Intersections::of(intersection).hit();

                if let Some(_) = hit {
                    canvas.set(x, HEIGHT.get() - y, Colour::RED)
                }
            }
        }
    }

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file")
}
