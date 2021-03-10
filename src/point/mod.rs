use crate::Vector3D;
use std::ops::{Add, Sub};
#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point3D(f64, f64, f64);

impl Point3D {
    pub const ORIGIN: Point3D = Point3D::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
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

    pub const fn w(&self) -> f64 {
        1.0
    }
}

impl From<(f64, f64, f64)> for Point3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Point3D(x, y, z)
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Point3D(self.0 + rhs.x(), self.1 + rhs.y(), self.2 + rhs.z())
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Vector3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Vector3D::new(self.0 - rhs.x(), self.1 - rhs.y(), self.2 - rhs.z())
    }
}

impl Sub<Vector3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Point3D(self.0 - rhs.x(), self.1 - rhs.y(), self.2 - rhs.z())
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use float_cmp::{ApproxEq, F64Margin};
    use proptest::prelude::*;

    impl Arbitrary for Point3D {
        type Parameters = ();

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                crate::util::reasonable_f64(),
                crate::util::reasonable_f64(),
                crate::util::reasonable_f64(),
            )
                .prop_map(|(x, y, z)| Point3D::new(x, y, z))
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }

    impl ApproxEq for Point3D {
        type Margin = F64Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();

            self.0.approx_eq(other.0, margin)
                && self.1.approx_eq(other.1, margin)
                && self.2.approx_eq(other.2, margin)
        }
    }
}
