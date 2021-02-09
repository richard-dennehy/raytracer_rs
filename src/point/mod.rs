use crate::Vector3D;
use std::ops::{Add, Sub};

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
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for Point3D {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Point3D::new(f64::arbitrary(g), f64::arbitrary(g), f64::arbitrary(g))
    }
}
