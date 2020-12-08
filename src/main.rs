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

mod ppm_writer;

fn main() {
    println!("Hello, world!");
}
