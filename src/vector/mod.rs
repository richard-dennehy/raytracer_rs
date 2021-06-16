use crate::Point3D;
use approx::AbsDiffEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3D(f64, f64, f64);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Normal3D(f64, f64, f64);

pub trait Vector: Sized + Copy + Clone {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64 {
        0.0
    }

    fn magnitude(&self) -> f64;
    fn normalised(&self) -> Normal3D;
    fn dot<V: Vector>(&self, other: V) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<V: Vector>(&self, other: V) -> Vector3D {
        Vector3D::new(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        )
    }

    fn reflect_through(&self, normal: Normal3D) -> Self;
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        debug_assert!(
            x.is_finite() && y.is_finite() && z.is_finite(),
            "`x`, `y`, and `z` components must always be finite"
        );
        Vector3D(x, y, z)
    }
}

impl From<Normal3D> for Vector3D {
    fn from(normal: Normal3D) -> Self {
        Vector3D(normal.x(), normal.y(), normal.z())
    }
}

impl Vector for Vector3D {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }

    fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    fn normalised(&self) -> Normal3D {
        let magnitude = self.magnitude();

        if magnitude <= f64::EPSILON {
            // this is wrong, but panicking isn't helpful, and it's difficult to replace this with a meaningful Unit vector
            // this may result in odd-looking colours for some pixels, but it _should_ be rare that this branch is ever hit (point of Cones)
            Normal3D::new(0.0, 0.0, 0.0)
        } else {
            Normal3D::new(
                self.x() / magnitude,
                self.y() / magnitude,
                self.z() / magnitude,
            )
        }
    }

    fn reflect_through(&self, normal: Normal3D) -> Self {
        *self - (normal * 2.0 * self.dot(normal))
    }
}

impl Normal3D {
    pub const POSITIVE_X: Normal3D = Normal3D::new(1.0, 0.0, 0.0);
    pub const NEGATIVE_X: Normal3D = Normal3D::new(-1.0, 0.0, 0.0);
    pub const POSITIVE_Y: Normal3D = Normal3D::new(0.0, 1.0, 0.0);
    pub const NEGATIVE_Y: Normal3D = Normal3D::new(0.0, -1.0, 0.0);
    pub const POSITIVE_Z: Normal3D = Normal3D::new(0.0, 0.0, 1.0);
    pub const NEGATIVE_Z: Normal3D = Normal3D::new(0.0, 0.0, -1.0);

    const fn new(x: f64, y: f64, z: f64) -> Self {
        Normal3D(x, y, z)
    }
}

impl Vector for Normal3D {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn magnitude(&self) -> f64 {
        1.0
    }

    fn normalised(&self) -> Normal3D {
        *self
    }

    fn reflect_through(&self, normal: Normal3D) -> Self {
        (*self - (normal * 2.0 * self.dot(normal))).normalised()
    }
}

impl From<(f64, f64, f64)> for Vector3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vector3D(x, y, z)
    }
}

impl<V: Vector> Add<V> for Vector3D {
    type Output = Vector3D;

    fn add(mut self, rhs: V) -> Self::Output {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();

        self
    }
}

impl<V: Vector> Add<V> for Normal3D {
    type Output = Vector3D;

    fn add(self, rhs: V) -> Self::Output {
        Vector3D::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Point3D> for Vector3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add<Point3D> for Normal3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl<V: Vector> Sub<V> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: V) -> Self::Output {
        Vector3D(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl<V: Vector> Sub<V> for Normal3D {
    type Output = Vector3D;

    fn sub(self, rhs: V) -> Self::Output {
        Vector3D(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;

        self
    }
}

impl Neg for Normal3D {
    type Output = Normal3D;

    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;

        self
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<f64> for Normal3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Div<f64> for Normal3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl AbsDiffEq for Normal3D {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON as f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
            && self.1.abs_diff_eq(&other.1, epsilon)
            && self.2.abs_diff_eq(&other.2, epsilon)
    }
}

impl AbsDiffEq for Vector3D {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON as f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
            && self.1.abs_diff_eq(&other.1, epsilon)
            && self.2.abs_diff_eq(&other.2, epsilon)
    }
}

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for Vector3D {
        type Parameters = ();

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                crate::util::reasonable_f64(),
                crate::util::reasonable_f64(),
                crate::util::reasonable_f64(),
            )
                .prop_map(|(x, y, z)| Vector3D::new(x, y, z))
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}
