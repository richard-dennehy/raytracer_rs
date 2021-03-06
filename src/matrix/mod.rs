use crate::{Point3D, Vector3D};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Mul, MulAssign};

#[cfg(test)]
mod tests;

mod underlying;
use underlying::*;

// TODO:
//   try storing just the inverse, and then inverting it when combining with other matrices,
//   as the inverse is generally what's used, and the normal form _should_ only be used in e.g. multiplication
//   - this would halve the size of the type
//   this really _really_ needs good property tests in place first
#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    underlying: Matrix4D,
    // TODO really need to write lots of tests to ensure this never falls out of sync
    // calculating the inverse is relatively expensive, bearing in mind matrices are inverted millions of times per render,
    // so pre-calculating the inverse has massive performance implications
    inverse: Option<Matrix4D>,
}

impl Transform {
    fn new(underlying: Matrix4D) -> Self {
        let inverse = underlying.inverse();

        Self {
            underlying,
            inverse,
        }
    }

    pub const fn identity() -> Self {
        Self {
            underlying: Matrix4D::identity(),
            inverse: Some(Matrix4D::identity()),
        }
    }

    pub fn translate_x(self, x: f64) -> Self {
        let translation = Transform::translation(x, 0.0, 0.0);

        translation * self
    }

    pub fn translate_y(self, y: f64) -> Self {
        let translation = Transform::translation(0.0, y, 0.0);

        translation * self
    }

    pub fn translate_z(self, z: f64) -> Self {
        let translation = Transform::translation(0.0, 0.0, z);

        translation * self
    }

    fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::new(Matrix4D::new(
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub fn scale_x(self, x: f64) -> Self {
        let transform = Self::scaling(x, 1.0, 1.0);

        transform * self
    }

    pub fn scale_y(self, y: f64) -> Self {
        let transform = Self::scaling(1.0, y, 1.0);

        transform * self
    }

    pub fn scale_z(self, z: f64) -> Self {
        let transform = Self::scaling(1.0, 1.0, z);

        transform * self
    }

    pub fn scale_all(self, factor: f64) -> Self {
        let transform = Self::scaling(factor, factor, factor);

        transform * self
    }

    fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::new(Matrix4D::new(
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub fn rotate_x(self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        let translation = Self::new(Matrix4D::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_r, -sin_r, 0.0],
            [0.0, sin_r, cos_r, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ));

        translation * self
    }

    #[rustfmt::skip]
    pub fn rotate_y(self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        let translation = Self::new(Matrix4D::new(
            [cos_r,  0.0, sin_r, 0.0],
            [0.0,    1.0,   0.0, 0.0],
            [-sin_r, 0.0, cos_r, 0.0],
            [0.0,    0.0,   0.0, 1.0],
        ));

        translation * self
    }

    #[rustfmt::skip]
    pub fn rotate_z(self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        let translation = Self::new(Matrix4D::new(
            [cos_r, -sin_r, 0.0, 0.0],
            [sin_r,  cos_r, 0.0, 0.0],
            [0.0,    0.0,   1.0, 0.0],
            [0.0,    0.0,   0.0, 1.0],
        ));

        translation * self
    }

    pub fn shear_x_to_y(self, shear: f64) -> Self {
        let transform = Self::shear(shear, 0.0, 0.0, 0.0, 0.0, 0.0);

        transform * self
    }

    pub fn shear_x_to_z(self, shear: f64) -> Self {
        let transform = Self::shear(0.0, shear, 0.0, 0.0, 0.0, 0.0);

        transform * self
    }

    pub fn shear_y_to_x(self, shear: f64) -> Self {
        let transform = Self::shear(0.0, 0.0, shear, 0.0, 0.0, 0.0);

        transform * self
    }

    pub fn shear_y_to_z(self, shear: f64) -> Self {
        let transform = Self::shear(0.0, 0.0, 0.0, shear, 0.0, 0.0);

        transform * self
    }

    pub fn shear_z_to_x(self, shear: f64) -> Self {
        let transform = Self::shear(0.0, 0.0, 0.0, 0.0, shear, 0.0);

        transform * self
    }

    pub fn shear_z_to_y(self, shear: f64) -> Self {
        let transform = Self::shear(0.0, 0.0, 0.0, 0.0, 0.0, shear);

        transform * self
    }

    #[rustfmt::skip]
    fn shear(
        x_proportionate_to_y: f64,
        x_proportionate_to_z: f64,
        y_proportionate_to_x: f64,
        y_proportionate_to_z: f64,
        z_proportionate_to_x: f64,
        z_proportionate_to_y: f64,
    ) -> Self {
        let x_to_y = x_proportionate_to_y;
        let x_to_z = x_proportionate_to_z;
        let y_to_x = y_proportionate_to_x;
        let y_to_z = y_proportionate_to_z;
        let z_to_x = z_proportionate_to_x;
        let z_to_y = z_proportionate_to_y;

        Self::new(Matrix4D::new(
            [1.0,    x_to_y, x_to_z, 0.0],
            [y_to_x, 1.0,    y_to_z, 0.0],
            [z_to_x, z_to_y, 1.0,    0.0],
            [0.0,    0.0,    0.0,    1.0],
        ))
    }

    pub fn inverse(&self) -> Option<Self> {
        debug_assert!(self.inverse == self.underlying.inverse());

        self.inverse.map(|inverse| Self {
            underlying: inverse,
            inverse: Some(self.underlying),
        })
    }

    pub fn transpose(&self) -> Self {
        Self::new(self.underlying.transpose())
    }

    pub fn view_transform(from: Point3D, to: Point3D, up: Vector3D) -> Self {
        let up = up.normalised();
        let forward = (to - from).normalised();
        let left = forward.cross(up);
        let true_up = left.cross(forward);

        let orientation = Self::new(Matrix4D::new(
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ));

        orientation * Transform::translation(-from.x(), -from.y(), -from.z())
    }
}

impl Debug for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let stringified = format!(
            "\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n",
            self.m00(), self.m01(), self.m02(), self.m03(),
            self.m10(), self.m11(), self.m12(), self.m13(),
            self.m20(), self.m21(), self.m22(), self.m23(),
            self.m30(), self.m31(), self.m32(), self.m33()
        );

        writeln!(f, "{}", stringified)
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(mut self, rhs: Transform) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<Transform> for Transform {
    fn mul_assign(&mut self, rhs: Transform) {
        self.underlying = self.underlying * rhs.underlying;
        // FIXME ideally wouldn't have to remember to do this manually
        self.inverse = self.underlying.inverse();
    }
}

impl Mul<Point3D> for &Transform {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        let (x, y, z, _) = self.underlying * rhs;

        Point3D::new(x, y, z)
    }
}

impl Mul<Vector3D> for &Transform {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let (x, y, z, _) = self.underlying * rhs;

        Vector3D::new(x, y, z)
    }
}

impl Mul<Vector3D> for Transform {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Point3D> for Transform {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        &self * rhs
    }
}

impl Transform {
    pub fn m00(&self) -> f64 {
        self.underlying.m00()
    }

    pub fn m01(&self) -> f64 {
        self.underlying.m01()
    }

    pub fn m02(&self) -> f64 {
        self.underlying.m02()
    }

    pub fn m03(&self) -> f64 {
        self.underlying.m03()
    }

    pub fn m10(&self) -> f64 {
        self.underlying.m10()
    }

    pub fn m11(&self) -> f64 {
        self.underlying.m11()
    }

    pub fn m12(&self) -> f64 {
        self.underlying.m12()
    }

    pub fn m13(&self) -> f64 {
        self.underlying.m13()
    }

    pub fn m20(&self) -> f64 {
        self.underlying.m20()
    }

    pub fn m21(&self) -> f64 {
        self.underlying.m21()
    }

    pub fn m22(&self) -> f64 {
        self.underlying.m22()
    }

    pub fn m23(&self) -> f64 {
        self.underlying.m23()
    }

    pub fn m30(&self) -> f64 {
        self.underlying.m30()
    }

    pub fn m31(&self) -> f64 {
        self.underlying.m31()
    }

    pub fn m32(&self) -> f64 {
        self.underlying.m32()
    }

    pub fn m33(&self) -> f64 {
        self.underlying.m33()
    }
}

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use crate::matrix::underlying::Matrix4D;
    use crate::Transform;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Transform {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Transform::new(Matrix4D::arbitrary(g))
        }
    }
}
