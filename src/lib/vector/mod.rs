use crate::Point3D;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3D(f64, f64, f64);

impl Vector3D {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D(x, y, z)
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
        0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalised(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.0 * other.x() + self.1 * other.y() + self.2 * other.z() // self.w + other.w is always 0
    }

    pub fn cross(&self, other: &Vector3D) -> Self {
        Vector3D(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        )
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

impl Div<f64> for &Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3D(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
impl Arbitrary for Vector3D {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Vector3D::new(f64::arbitrary(g), f64::arbitrary(g), f64::arbitrary(g))
    }
}