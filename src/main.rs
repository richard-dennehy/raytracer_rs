#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod point;
pub use point::Point3D;

mod vector;
pub use vector::Vector3D;

fn main() {
    println!("Hello, world!");
}
