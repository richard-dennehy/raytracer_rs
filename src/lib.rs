#[cfg(test)]
#[macro_use]
extern crate float_cmp;

mod point;
pub use point::Point3D;

mod vector;
pub use vector::*;

mod colour;
pub use colour::Colour;

mod canvas;
pub use canvas::*;

mod matrix;
pub use matrix::Transform;

pub mod ppm_writer;

mod ray;
pub use ray::{Intersection, Intersections, Ray};

mod light;
pub use light::Light;

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

pub mod obj_parser;

pub mod yaml_parser;

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

    #[cfg(test)]
    /// default f64 generator generates NaNs, enormous values, and minute values, all of which break
    /// the calculations and test assertions, and none of which are reasonable input values
    /// ("garbage in, garbage out" is a reasonable stance for a ray tracer)
    /// this restricts f64s to a reasonable but still fairly generous range
    pub fn reasonable_f64() -> std::ops::Range<f64> {
        -1000.0..1000.0
    }
}
