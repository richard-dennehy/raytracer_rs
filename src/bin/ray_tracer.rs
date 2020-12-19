extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::fs;
use std::num::NonZeroU16;

const WIDTH: NonZeroU16 = nonzero!(800u16);
const HEIGHT: NonZeroU16 = nonzero!(800u16);
const WALL_Z: f64 = 1.1;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT).unwrap();
    let mut sphere = Sphere::unit();
    sphere.transform(Matrix4D::translation(0.0, 20.0, 0.0));

    let ray_origin = Point3D::new(0.0, 0.0, -1.1);

    for x in 0..WIDTH.get() {
        for y in 0..HEIGHT.get() {
            let world_x = x as i16 - ((WIDTH.get() / 2) as i16);
            let world_y = ((HEIGHT.get() / 2) as i16) - y as i16;

            let target = Point3D::new(world_x as _, world_y as _, WALL_Z);

            let ray = Ray::new(ray_origin, (target - ray_origin).normalised());
            let intersection = ray.intersect(&sphere);
            if let Some(intersection) = intersection {
                let hit = Intersections::of(intersection).hit();

                if let Some(_) = hit {
                    canvas.set(x, y, Colour::RED)
                }
            }
        }
    }

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file")
}
