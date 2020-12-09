#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
#[macro_use]
extern crate float_cmp;

mod point;
pub use point::Point3D;

mod vector;
pub use vector::Vector3D;

mod colour;
pub use colour::Colour;

mod canvas;
pub use canvas::*;

use std::fs;
use std::num::NonZeroU16;

mod ppm_writer;

fn main() {
    let environment = Environment {
        gravity: Vector3D::new(0.0, -0.08, 0.0),
        wind: Vector3D::new(-0.01, 0.0, 0.0),
    };
    let mut projectile = Projectile {
        position: Point3D::new(0.0, 1.0, 0.0),
        velocity: Vector3D::new(1.0, 1.8, 0.0).normalised() * 10.0,
    };

    let mut canvas =
        Canvas::new(NonZeroU16::new(800).unwrap(), NonZeroU16::new(600).unwrap()).unwrap();

    while projectile.position.y() >= 0.0 {
        projectile.position = projectile.position + projectile.velocity;
        projectile.velocity = projectile.velocity + environment.gravity + environment.wind;

        let x = projectile.position.x().round() as isize;
        let y = projectile.position.y().round() as isize;

        if x >= 0 && x < canvas.width() as _ && y >= 0 && y < canvas.height() as _ {
            let x = x as u16;
            let y = (canvas.height() - y as usize) as u16;

            canvas.set(x, y, Colour::RED);
        }
    }

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file")
}

struct Environment {
    gravity: Vector3D,
    wind: Vector3D,
}

struct Projectile {
    position: Point3D,
    velocity: Vector3D,
}
