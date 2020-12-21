extern crate ray_tracer;

#[macro_use]
extern crate nonzero_ext;

use ray_tracer::*;
use std::fs;
use std::num::NonZeroU16;

const CANVAS_SIZE: NonZeroU16 = nonzero!(800u16);
const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;

fn main() {
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE).unwrap();
    let mut sphere = Sphere::unit();
    sphere.material.colour = Colour::new(1.0, 0.2, 1.0);

    let light = PointLight::new(Colour::WHITE, Point3D::new(-10.0, 10.0, -10.0));

    let ray_origin = Point3D::new(0.0, 0.0, -5.0);
    let pixel_size = WALL_SIZE / (CANVAS_SIZE.get() as f64);
    let half = WALL_SIZE / 2.0;

    for x in 0..CANVAS_SIZE.get() {
        for y in 0..CANVAS_SIZE.get() {
            let world_x = -half + pixel_size * x as f64;
            let world_y = half - pixel_size * y as f64;

            let target = Point3D::new(world_x, world_y, WALL_Z);

            let ray = Ray::new(ray_origin, (target - ray_origin).normalised());
            let intersection = ray.intersect(&sphere);
            if let Some((first, second)) = intersection {
                let hit = Intersections::of(first, second).hit();

                if let Some(hit) = hit {
                    let position = ray.position(hit.t);

                    let eye_vector = -ray.direction;

                    let colour = hit.with.colour_at(position, &light, eye_vector);
                    canvas.set(x, y, colour)
                }
            }
        }
    }

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file")
}
