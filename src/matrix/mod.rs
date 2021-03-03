use crate::{Point3D, Vector3D};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Mul, MulAssign};

#[cfg(test)]
mod tests;

mod underlying;
use underlying::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    underlying: Matrix4D,
}

impl Transform {
    const fn new(underlying: Matrix4D) -> Self {
        Transform { underlying }
    }

    pub const fn identity() -> Self {
        Self::new(Matrix4D::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub const fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::new(Matrix4D::new(
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub fn with_translation(self, x: f64, y: f64, z: f64) -> Self {
        let translation = Transform::translation(x, y, z);

        translation * self
    }

    pub const fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::new(Matrix4D::new(
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub const fn uniform_scaling(scale: f64) -> Self {
        Self::scaling(scale, scale, scale)
    }

    pub fn with_scaling(self, x: f64, y: f64, z: f64) -> Self {
        let scaling = Transform::scaling(x, y, z);

        scaling * self
    }

    pub fn rotation_x(radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

        Self::new(Matrix4D::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_r, -sin_r, 0.0],
            [0.0, sin_r, cos_r, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ))
    }

    pub fn with_rotation_x(self, radians: f64) -> Self {
        let rotation_x = Transform::rotation_x(radians);

        rotation_x * self
    }

    #[rustfmt::skip]
    pub fn rotation_y(radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

Self::new(        Matrix4D::new(
            [cos_r,  0.0, sin_r, 0.0],
            [0.0,    1.0,   0.0, 0.0],
            [-sin_r, 0.0, cos_r, 0.0],
            [0.0,    0.0,   0.0, 1.0],
        )
)    }

    pub fn with_rotation_y(self, radians: f64) -> Self {
        let rotation_y = Transform::rotation_y(radians);

        rotation_y * self
    }

    #[rustfmt::skip]
    pub fn rotation_z(radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();

Self::new(        Matrix4D::new(
            [cos_r, -sin_r, 0.0, 0.0],
            [sin_r,  cos_r, 0.0, 0.0],
            [0.0,    0.0,   1.0, 0.0],
            [0.0,    0.0,   0.0, 1.0],
        )
)    }

    pub fn with_rotation_z(self, radians: f64) -> Self {
        let rotation_z = Transform::rotation_z(radians);

        rotation_z * self
    }

    #[rustfmt::skip]
    pub fn shear(
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

Self::new(        Matrix4D::new(
            [1.0,    x_to_y, x_to_z, 0.0],
            [y_to_x, 1.0,    y_to_z, 0.0],
            [z_to_x, z_to_y, 1.0,    0.0],
            [0.0,    0.0,    0.0,    1.0],
        )
)    }

    pub fn with_shear(
        self,
        x_proportionate_to_y: f64,
        x_proportionate_to_z: f64,
        y_proportionate_to_x: f64,
        y_proportionate_to_z: f64,
        z_proportionate_to_x: f64,
        z_proportionate_to_y: f64,
    ) -> Self {
        let shear = Transform::shear(
            x_proportionate_to_y,
            x_proportionate_to_z,
            y_proportionate_to_x,
            y_proportionate_to_z,
            z_proportionate_to_x,
            z_proportionate_to_y,
        );

        shear * self
    }

    pub fn inverse(&self) -> Option<Self> {
        self.underlying.inverse().map(Self::new)
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
            "| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n",
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
        self.underlying = self.underlying * rhs.underlying
    }
}

impl Mul<Point3D> for &Transform {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: Point3D) -> Self::Output {
        (
            self.m00() * rhs.x() + self.m01() * rhs.y() + self.m02() * rhs.z() + self.m03(),
            self.m10() * rhs.x() + self.m11() * rhs.y() + self.m12() * rhs.z() + self.m13(),
            self.m20() * rhs.x() + self.m21() * rhs.y() + self.m22() * rhs.z() + self.m23(),
            self.m30() * rhs.x() + self.m31() * rhs.y() + self.m32() * rhs.z() + self.m33(),
        )
    }
}

impl Mul<Vector3D> for &Transform {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: Vector3D) -> Self::Output {
        (
            self.m00() * rhs.x() + self.m01() * rhs.y() + self.m02() * rhs.z(),
            self.m10() * rhs.x() + self.m11() * rhs.y() + self.m12() * rhs.z(),
            self.m20() * rhs.x() + self.m21() * rhs.y() + self.m22() * rhs.z(),
            self.m30() * rhs.x() + self.m31() * rhs.y() + self.m32() * rhs.z(),
        )
    }
}

impl Mul<Vector3D> for Transform {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: Vector3D) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Point3D> for Transform {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: Point3D) -> Self::Output {
        &self * rhs
    }
}

impl Mul<(f64, f64, f64, f64)> for Transform {
    type Output = (f64, f64, f64, f64);

    fn mul(self, (x, y, z, w): (f64, f64, f64, f64)) -> Self::Output {
        (
            self.m00() * x + self.m01() * y + self.m02() * z + self.m03() * w,
            self.m10() * x + self.m11() * y + self.m12() * z + self.m13() * w,
            self.m20() * x + self.m21() * y + self.m22() * z + self.m23() * w,
            self.m30() * x + self.m31() * y + self.m32() * z + self.m33() * w,
        )
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
