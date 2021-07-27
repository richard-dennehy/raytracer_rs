use super::{Vector, Vector3D};
use approx::AbsDiffEq;
use std::ops::{Add, Sub};
#[cfg(test)]
pub use test_utils::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point3D(f64, f64, f64);

impl Point3D {
    pub const ORIGIN: Point3D = Point3D(0.0, 0.0, 0.0);

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D(x, y, z)
    }

    pub const fn x(&self) -> f64 {
        self.0
    }

    pub const fn y(&self) -> f64 {
        self.1
    }

    pub const fn z(&self) -> f64 {
        self.2
    }

    /// returns a new 3D Point with the minimum `x`, `y`, and `z` of the provided points
    ///
    /// `points` must not be empty
    pub fn min<const N: usize>(points: [Point3D; N]) -> Point3D {
        assert!(
            points.len() >= 1,
            "cannot find the minimum of an empty list of Points"
        );

        let first = points[0];
        std::array::IntoIter::new(points)
            .skip(1)
            .fold(first, |acc, next| {
                Point3D::new(
                    acc.x().min(next.x()),
                    acc.y().min(next.y()),
                    acc.z().min(next.z()),
                )
            })
    }

    /// returns a new 3D Point with the maximum `x`, `y`, and `z` of the provided points
    ///
    /// `points` must not be empty
    pub fn max<const N: usize>(points: [Point3D; N]) -> Point3D {
        assert!(
            points.len() >= 1,
            "cannot find the maximum of an empty list of Points"
        );

        let first = points[0];
        std::array::IntoIter::new(points)
            .skip(1)
            .fold(first, |acc, next| {
                Point3D::new(
                    acc.x().max(next.x()),
                    acc.y().max(next.y()),
                    acc.z().max(next.z()),
                )
            })
    }
}

impl From<(f64, f64, f64)> for Point3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Point3D(x, y, z)
    }
}

impl<V: Vector> Add<V> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: V) -> Self::Output {
        Point3D(self.0 + rhs.x(), self.1 + rhs.y(), self.2 + rhs.z())
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Vector3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Vector3D::new(self.0 - rhs.x(), self.1 - rhs.y(), self.2 - rhs.z())
    }
}

impl<V: Vector> Sub<V> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: V) -> Self::Output {
        Point3D(self.0 - rhs.x(), self.1 - rhs.y(), self.2 - rhs.z())
    }
}

impl AbsDiffEq for Point3D {
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
mod test_utils {
    use crate::core::Point3D;
    use quickcheck::{Arbitrary, Gen};
    use rand::prelude::*;

    impl Arbitrary for Point3D {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = thread_rng();
            fn gen_component(rng: &mut ThreadRng) -> f64 {
                rng.gen_range(-10.0..10.0)
            }

            Self::new(
                gen_component(&mut rng),
                gen_component(&mut rng),
                gen_component(&mut rng),
            )
        }
    }
}
