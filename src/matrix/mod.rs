use crate::{Normal3D, Point3D, Vector, Vector3D};
use approx::AbsDiffEq;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Mul, MulAssign};

#[cfg(test)]
mod tests;

mod underlying;
pub use underlying::Matrix4D;

#[derive(PartialEq, Clone, Copy)]
pub struct Transform {
    // calculating the inverse is relatively expensive, bearing in mind matrices are inverted millions of times per render,
    // so pre-calculating the inverse has massive performance implications
    inverse: Matrix4D,
}

impl Transform {
    fn new(underlying: Matrix4D) -> Self {
        let inverse = underlying
            .inverse()
            .expect("transformation matrix is not invertible");

        Self { inverse }
    }

    pub const fn identity() -> Self {
        Self {
            inverse: Matrix4D::identity(),
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
        assert!(
            x != 0.0 && y != 0.0 && z != 0.0,
            "cannot scale to 0 (not invertible)"
        );

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

    // shear operations - only allow shearing in one axis at a time, as shearing in multiple axes simultaneously is not necessarily invertible

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

    pub fn inverse(&self) -> Matrix4D {
        self.inverse
    }

    pub fn view_transform(eye: Point3D, target: Point3D, up: Normal3D) -> Self {
        let forward = (target - eye).normalised();
        let left = forward.cross(up);
        let true_up = left.cross(forward);

        let orientation = Self::new(Matrix4D::new(
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ));

        orientation * Transform::translation(-eye.x(), -eye.y(), -eye.z())
    }
}

impl Debug for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let underlying = self.inverse.inverse().unwrap();

        let stringified = format!(
            "\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n| {} | {} | {} | {} |\n",
            underlying.m00(), underlying.m01(), underlying.m02(), underlying.m03(),
            underlying.m10(), underlying.m11(), underlying.m12(), underlying.m13(),
            underlying.m20(), underlying.m21(), underlying.m22(), underlying.m23(),
            underlying.m30(), underlying.m31(), underlying.m32(), underlying.m33()
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
        self.inverse = rhs.inverse * self.inverse;
    }
}

impl Mul<Point3D> for &Transform {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        let (x, y, z, _) = self.inverse.inverse().unwrap() * rhs;

        Point3D::new(x, y, z)
    }
}

impl Mul<Vector3D> for &Transform {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let (x, y, z, _) = self.inverse.inverse().unwrap() * rhs;

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

impl AbsDiffEq for Transform {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON as f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.inverse.abs_diff_eq(&other.inverse, epsilon)
    }
}

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use crate::matrix::underlying::Matrix4D;
    use crate::Transform;
    use proptest::collection;
    use proptest::option;
    use proptest::prelude::*;
    use std::f64::consts::PI;

    impl Transform {
        pub fn underlying(&self) -> Matrix4D {
            self.inverse.inverse().unwrap()
        }

        pub fn any_transform() -> BoxedStrategy<Self> {
            fn any_single_transform() -> BoxedStrategy<Transform> {
                proptest::prop_oneof![
                    Transform::any_translation(),
                    Transform::any_scaling(),
                    Transform::any_shear(),
                    Transform::any_rotation(),
                ]
                .boxed()
            }

            collection::vec(any_single_transform(), 1..5)
                .prop_map(|transforms| {
                    transforms
                        .into_iter()
                        .fold(Transform::identity(), |acc, next| next * acc)
                })
                .boxed()
        }

        pub fn any_translation() -> BoxedStrategy<Self> {
            (
                option::of(-5.0..5.0),
                option::of(-5.0..5.0),
                option::of(-5.0..5.0),
            )
                .prop_filter("zero translation", |(x, y, z)| {
                    x.is_none() && y.is_none() && z.is_none()
                })
                .prop_map(|(x, y, z)| {
                    Transform::identity()
                        .translate_x(x.unwrap_or(0.0))
                        .translate_y(y.unwrap_or(0.0))
                        .translate_z(z.unwrap_or(0.0))
                })
                .boxed()
        }

        pub fn any_scaling() -> BoxedStrategy<Self> {
            // use smallish values to stop combined transforms from generating enormous values (and accumulating enormous rounding errors)
            (
                option::of(-1.5..1.5),
                option::of(-1.5..1.5),
                option::of(-1.5..1.5),
            )
                .prop_filter("no scaling", |(x, y, z)| {
                    x.is_none() && y.is_none() && z.is_none()
                })
                .prop_map(|(x, y, z)| {
                    Transform::identity()
                        .translate_x(x.unwrap_or(1.0))
                        .translate_y(y.unwrap_or(1.0))
                        .translate_z(z.unwrap_or(1.0))
                })
                .boxed()
        }

        pub fn any_shear() -> BoxedStrategy<Self> {
            proptest::prop_oneof![
                (-1.5..1.5).prop_map(|x_to_y| Transform::identity().shear_x_to_y(x_to_y)),
                (-1.5..1.5).prop_map(|x_to_z| Transform::identity().shear_x_to_z(x_to_z)),
                (-1.5..1.5).prop_map(|y_to_x| Transform::identity().shear_y_to_x(y_to_x)),
                (-1.5..1.5).prop_map(|y_to_z| Transform::identity().shear_y_to_z(y_to_z)),
                (-1.5..1.5).prop_map(|z_to_x| Transform::identity().shear_z_to_x(z_to_x)),
                (-1.5..1.5).prop_map(|z_to_y| Transform::identity().shear_z_to_y(z_to_y)),
            ]
            .boxed()
        }

        pub fn any_rotation() -> BoxedStrategy<Self> {
            // -2π to 2π radians = -360deg to 360deg
            fn radians() -> BoxedStrategy<f64> {
                (-2.0..2.0).prop_map(|r| r * PI).boxed()
            }

            (
                option::of(radians()),
                option::of(radians()),
                option::of(radians()),
            )
                .prop_filter("no rotation", |(x, y, z)| {
                    x.is_none() && y.is_none() && z.is_none()
                })
                .prop_map(|(x, y, z)| {
                    Transform::identity()
                        .rotate_x(x.unwrap_or(0.0))
                        .rotate_y(y.unwrap_or(0.0))
                        .rotate_z(z.unwrap_or(0.0))
                })
                .boxed()
        }
    }
}
