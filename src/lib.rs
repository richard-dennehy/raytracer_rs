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

mod matrix;
pub use matrix::Matrix4D;

pub mod ppm_writer;

mod ray;
pub use ray::{Intersection, Intersections, Ray};

mod light;
pub use light::PointLight;

mod material;
pub use material::Material;

mod world;
pub use world::World;

mod camera;
pub use camera::Camera;

pub mod renderer;

mod object;
pub use object::Object;

mod pattern;
pub use pattern::Pattern;

pub mod image_writer;

pub mod util {
    pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        };

        let first = (-b - discriminant.sqrt()) / (2.0 * a);
        let second = (-b + discriminant.sqrt()) / (2.0 * a);

        Some((first, second))
    }
}
