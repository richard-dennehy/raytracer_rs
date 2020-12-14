#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[cfg(test)]
#[macro_use]
extern crate float_cmp;

#[macro_use]
extern crate nonzero_ext;

mod point;
pub use point::Point3D;

mod vector;
pub use vector::Vector3D;

mod colour;
pub use colour::Colour;

mod canvas;
pub use canvas::*;

use std::fs;

mod ppm_writer;

mod matrix;
pub use matrix::Matrix4D;
use std::f64::consts::PI;

fn main() {
    let mut canvas = Canvas::new(nonzero!(800u16), nonzero!(800u16)).unwrap();

    let point = Point3D::new(0.0, 0.0, 0.0);
    let diameter = 350.0;
    let centre = (400.0, 400.0);

    for i in 0..12 {
        let angle = ((2.0 * PI) / 12.0) * i as f64;

        let transform = Matrix4D::translation(0.0, diameter, 0.0)
            .with_rotation_z(-angle) // rotation is anti-clockwise
            .with_translation(centre.0, centre.1, 0.0);

        let (x, y, _, _) = transform * point;

        // paint 3x3 "pixel"
        for dx in 0..2 {
            for dy in 0..2 {
                let x = (x.round() as i16 + (dx - 1)) as u16;
                let y = (799 - (y.round() as i16 + (dy - 1))) as u16;

                canvas.set(x, y, Colour::WHITE);
            }
        }
    }

    let ppm_content = ppm_writer::write_ppm(&canvas);

    fs::write("out.ppm", ppm_content).expect("Failed to write output file")
}
