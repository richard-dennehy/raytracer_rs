mod point;
pub use point::Point3D;

mod vector;
pub use vector::{Normal3D, Vector, Vector3D};

mod matrix;
pub use matrix::Matrix4D;

mod transform;
pub use transform::Transform;

mod colour;
pub use colour::Colour;

mod ray;
pub use ray::Ray;
pub use ray::{HitData, Intersection, Intersections};

pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    };

    let first = (-b - discriminant.sqrt()) / (2.0 * a);
    let second = (-b + discriminant.sqrt()) / (2.0 * a);

    Some((first, second))
}

pub trait F64Ext {
    fn roughly_equals(&self, other: Self) -> bool;
    fn is_roughly_gte(&self, other: Self) -> bool;
    fn is_roughly_lte(&self, other: Self) -> bool;
    fn is_roughly_zero(&self) -> bool;
    fn is_not_roughly_zero(&self) -> bool {
        !self.is_roughly_zero()
    }
}

// actual f64 epsilon isn't nearly lenient enough
pub const EPSILON: f64 = f32::EPSILON as f64;

impl F64Ext for f64 {
    fn roughly_equals(&self, other: Self) -> bool {
        approx::relative_eq!(self, &other, epsilon = EPSILON)
    }

    fn is_roughly_gte(&self, other: Self) -> bool {
        *self >= other - EPSILON
    }

    fn is_roughly_lte(&self, other: Self) -> bool {
        *self <= other + EPSILON
    }

    fn is_roughly_zero(&self) -> bool {
        self.roughly_equals(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod colour_tests;
    mod matrix_tests;
    mod point_tests;
    mod ray_tests;
    mod transform_tests;
    mod vector_tests;
}
