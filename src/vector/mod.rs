use crate::Point3D;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3D(f64, f64, f64);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Normal3D(f64, f64, f64);

pub trait Vector {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64 {
        0.0
    }

    fn magnitude(&self) -> f64;
    fn normalised(&self) -> Normal3D;
    fn dot<V: Vector>(&self, other: V) -> f64 {
        self.0 * other.x() + self.1 * other.y() + self.2 * other.z()
    }

    fn cross<V: Vector>(&self, other: V) -> Vector3D {
        Vector3D::new(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        )
    }
}

impl Vector3D {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D(x, y, z)
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
            Normal3D::new(0.0, 0.0, 0.0) // FIXME this is wrong - probably just panic here instead
        } else {
            Normal3D::new(
                self.x() / magnitude,
                self.y() / magnitude,
                self.z() / magnitude,
            )
        }
    }
}

impl Normal3D {
    const fn new(x: f64, y: f64, z: f64) -> Self {
        Normal3D(x, y, z)
    }

    pub fn reflect_through(&self, normal: Normal3D) -> Self {
        *self - (normal * 2.0 * self.dot(normal))
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
}

impl From<(f64, f64, f64)> for Vector3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vector3D(x, y, z)
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(mut self, rhs: Vector3D) -> Self::Output {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();

        self
    }
}

impl Add<Point3D> for Vector3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.0 + rhs.x(), self.1 + rhs.y(), self.2 + rhs.z())
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
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

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use super::*;
    use float_cmp::{ApproxEq, F64Margin};
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

    impl ApproxEq for Vector3D {
        type Margin = F64Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();

            self.0.approx_eq(other.0, margin)
                && self.1.approx_eq(other.1, margin)
                && self.2.approx_eq(other.2, margin)
        }
    }
}
