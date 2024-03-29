use crate::core::Point3D;
use crate::core::VectorMaths;
use approx::AbsDiffEq;
use std::ops::{Mul, MulAssign};

#[cfg(test)]
pub use test_utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix4D {
    underlying: [[f64; 4]; 4],
}

impl Matrix4D {
    pub(super) const fn new(
        row0: [f64; 4],
        row1: [f64; 4],
        row2: [f64; 4],
        row3: [f64; 4],
    ) -> Self {
        Matrix4D {
            underlying: [row0, row1, row2, row3],
        }
    }

    pub(super) const fn identity() -> Self {
        Matrix4D::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn transpose(&self) -> Self {
        Matrix4D::new(
            [self.m00(), self.m10(), self.m20(), self.m30()],
            [self.m01(), self.m11(), self.m21(), self.m31()],
            [self.m02(), self.m12(), self.m22(), self.m32()],
            [self.m03(), self.m13(), self.m23(), self.m33()],
        )
    }

    pub(super) fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return None;
        }

        // build transposed cofactor matrix, with all elements divided by determinant, in one set of operations
        // (as opposed to building a cofactor matrix, calling `transpose`, and then dividing all elements)
        // n.b. could optimise by avoiding repetition of cofactor(0, 0), (0,1), (0,2), (0, 3) calculation
        let row0 = [
            self.cofactor(0, 0) / determinant,
            self.cofactor(1, 0) / determinant,
            self.cofactor(2, 0) / determinant,
            self.cofactor(3, 0) / determinant,
        ];
        let row1 = [
            self.cofactor(0, 1) / determinant,
            self.cofactor(1, 1) / determinant,
            self.cofactor(2, 1) / determinant,
            self.cofactor(3, 1) / determinant,
        ];
        let row2 = [
            self.cofactor(0, 2) / determinant,
            self.cofactor(1, 2) / determinant,
            self.cofactor(2, 2) / determinant,
            self.cofactor(3, 2) / determinant,
        ];
        let row3 = [
            self.cofactor(0, 3) / determinant,
            self.cofactor(1, 3) / determinant,
            self.cofactor(2, 3) / determinant,
            self.cofactor(3, 3) / determinant,
        ];

        Some(Matrix4D::new(row0, row1, row2, row3))
    }

    pub(in crate::core) fn determinant(&self) -> f64 {
        self.m00() * self.cofactor(0, 0)
            + self.m01() * self.cofactor(0, 1)
            + self.m02() * self.cofactor(0, 2)
            + self.m03() * self.cofactor(0, 3)
    }

    pub(in crate::core) fn cofactor(&self, row: u8, column: u8) -> f64 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn minor(&self, row: u8, column: u8) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub(in crate::core) fn submatrix(&self, excluding_row: u8, excluding_column: u8) -> Matrix3D {
        match (excluding_row, excluding_column) {
            (0, 0) => Matrix3D::new(
                [self.m11(), self.m12(), self.m13()],
                [self.m21(), self.m22(), self.m23()],
                [self.m31(), self.m32(), self.m33()],
            ),
            (0, 1) => Matrix3D::new(
                [self.m10(), self.m12(), self.m13()],
                [self.m20(), self.m22(), self.m23()],
                [self.m30(), self.m32(), self.m33()],
            ),
            (0, 2) => Matrix3D::new(
                [self.m10(), self.m11(), self.m13()],
                [self.m20(), self.m21(), self.m23()],
                [self.m30(), self.m31(), self.m33()],
            ),
            (0, 3) => Matrix3D::new(
                [self.m10(), self.m11(), self.m12()],
                [self.m20(), self.m21(), self.m22()],
                [self.m30(), self.m31(), self.m32()],
            ),
            (1, 0) => Matrix3D::new(
                [self.m01(), self.m02(), self.m03()],
                [self.m21(), self.m22(), self.m23()],
                [self.m31(), self.m32(), self.m33()],
            ),
            (1, 1) => Matrix3D::new(
                [self.m00(), self.m02(), self.m03()],
                [self.m20(), self.m22(), self.m23()],
                [self.m30(), self.m32(), self.m33()],
            ),
            (1, 2) => Matrix3D::new(
                [self.m00(), self.m01(), self.m03()],
                [self.m20(), self.m21(), self.m23()],
                [self.m30(), self.m31(), self.m33()],
            ),
            (1, 3) => Matrix3D::new(
                [self.m00(), self.m01(), self.m02()],
                [self.m20(), self.m21(), self.m22()],
                [self.m30(), self.m31(), self.m32()],
            ),
            (2, 0) => Matrix3D::new(
                [self.m01(), self.m02(), self.m03()],
                [self.m11(), self.m12(), self.m13()],
                [self.m31(), self.m32(), self.m33()],
            ),
            (2, 1) => Matrix3D::new(
                [self.m00(), self.m02(), self.m03()],
                [self.m10(), self.m12(), self.m13()],
                [self.m30(), self.m32(), self.m33()],
            ),
            (2, 2) => Matrix3D::new(
                [self.m00(), self.m01(), self.m03()],
                [self.m10(), self.m11(), self.m13()],
                [self.m30(), self.m31(), self.m33()],
            ),
            (2, 3) => Matrix3D::new(
                [self.m00(), self.m01(), self.m02()],
                [self.m10(), self.m11(), self.m12()],
                [self.m30(), self.m31(), self.m32()],
            ),
            (3, 0) => Matrix3D::new(
                [self.m01(), self.m02(), self.m03()],
                [self.m11(), self.m12(), self.m13()],
                [self.m21(), self.m22(), self.m23()],
            ),
            (3, 1) => Matrix3D::new(
                [self.m00(), self.m02(), self.m03()],
                [self.m10(), self.m12(), self.m13()],
                [self.m20(), self.m22(), self.m23()],
            ),
            (3, 2) => Matrix3D::new(
                [self.m00(), self.m01(), self.m03()],
                [self.m10(), self.m11(), self.m13()],
                [self.m20(), self.m21(), self.m23()],
            ),
            (3, 3) => Matrix3D::new(
                [self.m00(), self.m01(), self.m02()],
                [self.m10(), self.m11(), self.m12()],
                [self.m20(), self.m21(), self.m22()],
            ),
            _ => panic!(
                "invalid 4D matrix row {} and column {}",
                excluding_row, excluding_column
            ),
        }
    }
}

impl Mul<Matrix4D> for Matrix4D {
    type Output = Matrix4D;

    fn mul(mut self, rhs: Matrix4D) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<Matrix4D> for Matrix4D {
    fn mul_assign(&mut self, rhs: Matrix4D) {
        self.underlying[0] = [
            self.m00() * rhs.m00()
                + self.m01() * rhs.m10()
                + self.m02() * rhs.m20()
                + self.m03() * rhs.m30(),
            self.m00() * rhs.m01()
                + self.m01() * rhs.m11()
                + self.m02() * rhs.m21()
                + self.m03() * rhs.m31(),
            self.m00() * rhs.m02()
                + self.m01() * rhs.m12()
                + self.m02() * rhs.m22()
                + self.m03() * rhs.m32(),
            self.m00() * rhs.m03()
                + self.m01() * rhs.m13()
                + self.m02() * rhs.m23()
                + self.m03() * rhs.m33(),
        ];
        self.underlying[1] = [
            self.m10() * rhs.m00()
                + self.m11() * rhs.m10()
                + self.m12() * rhs.m20()
                + self.m13() * rhs.m30(),
            self.m10() * rhs.m01()
                + self.m11() * rhs.m11()
                + self.m12() * rhs.m21()
                + self.m13() * rhs.m31(),
            self.m10() * rhs.m02()
                + self.m11() * rhs.m12()
                + self.m12() * rhs.m22()
                + self.m13() * rhs.m32(),
            self.m10() * rhs.m03()
                + self.m11() * rhs.m13()
                + self.m12() * rhs.m23()
                + self.m13() * rhs.m33(),
        ];
        self.underlying[2] = [
            self.m20() * rhs.m00()
                + self.m21() * rhs.m10()
                + self.m22() * rhs.m20()
                + self.m23() * rhs.m30(),
            self.m20() * rhs.m01()
                + self.m21() * rhs.m11()
                + self.m22() * rhs.m21()
                + self.m23() * rhs.m31(),
            self.m20() * rhs.m02()
                + self.m21() * rhs.m12()
                + self.m22() * rhs.m22()
                + self.m23() * rhs.m32(),
            self.m20() * rhs.m03()
                + self.m21() * rhs.m13()
                + self.m22() * rhs.m23()
                + self.m23() * rhs.m33(),
        ];
        self.underlying[3] = [
            self.m30() * rhs.m00()
                + self.m31() * rhs.m10()
                + self.m32() * rhs.m20()
                + self.m33() * rhs.m30(),
            self.m30() * rhs.m01()
                + self.m31() * rhs.m11()
                + self.m32() * rhs.m21()
                + self.m33() * rhs.m31(),
            self.m30() * rhs.m02()
                + self.m31() * rhs.m12()
                + self.m32() * rhs.m22()
                + self.m33() * rhs.m32(),
            self.m30() * rhs.m03()
                + self.m31() * rhs.m13()
                + self.m32() * rhs.m23()
                + self.m33() * rhs.m33(),
        ]
    }
}

impl Mul<Point3D> for &Matrix4D {
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

impl<V: VectorMaths> Mul<V> for &Matrix4D {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: V) -> Self::Output {
        (
            self.m00() * rhs.x() + self.m01() * rhs.y() + self.m02() * rhs.z(),
            self.m10() * rhs.x() + self.m11() * rhs.y() + self.m12() * rhs.z(),
            self.m20() * rhs.x() + self.m21() * rhs.y() + self.m22() * rhs.z(),
            self.m30() * rhs.x() + self.m31() * rhs.y() + self.m32() * rhs.z(),
        )
    }
}

impl<V: VectorMaths> Mul<V> for Matrix4D {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: V) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Point3D> for Matrix4D {
    type Output = (f64, f64, f64, f64);

    fn mul(self, rhs: Point3D) -> Self::Output {
        &self * rhs
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(in crate::core) struct Matrix3D {
    underlying: [[f64; 3]; 3],
}

impl Matrix3D {
    pub fn new(row0: [f64; 3], row1: [f64; 3], row2: [f64; 3]) -> Self {
        Matrix3D {
            underlying: [row0, row1, row2],
        }
    }

    pub(crate) fn determinant(&self) -> f64 {
        self.m00() * self.cofactor(0, 0)
            + self.m01() * self.cofactor(0, 1)
            + self.m02() * self.cofactor(0, 2)
    }

    pub(crate) fn cofactor(&self, row: u8, column: u8) -> f64 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub(crate) fn minor(&self, row: u8, column: u8) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub(crate) fn submatrix(&self, excluding_row: u8, excluding_column: u8) -> Matrix2D {
        match (excluding_row, excluding_column) {
            (0, 0) => Matrix2D::new([self.m11(), self.m12()], [self.m21(), self.m22()]),
            (0, 1) => Matrix2D::new([self.m10(), self.m12()], [self.m20(), self.m22()]),
            (0, 2) => Matrix2D::new([self.m10(), self.m11()], [self.m20(), self.m21()]),
            (1, 0) => Matrix2D::new([self.m01(), self.m02()], [self.m21(), self.m22()]),
            (1, 1) => Matrix2D::new([self.m00(), self.m02()], [self.m20(), self.m22()]),
            (1, 2) => Matrix2D::new([self.m00(), self.m01()], [self.m20(), self.m21()]),
            (2, 0) => Matrix2D::new([self.m01(), self.m02()], [self.m11(), self.m12()]),
            (2, 1) => Matrix2D::new([self.m00(), self.m02()], [self.m10(), self.m12()]),
            (2, 2) => Matrix2D::new([self.m00(), self.m01()], [self.m10(), self.m11()]),
            _ => panic!(
                "invalid 3D matrix row {} and column {}",
                excluding_row, excluding_column
            ),
        }
    }

    pub(crate) fn m00(&self) -> f64 {
        self.underlying[0][0]
    }

    fn m01(&self) -> f64 {
        self.underlying[0][1]
    }

    fn m02(&self) -> f64 {
        self.underlying[0][2]
    }

    fn m10(&self) -> f64 {
        self.underlying[1][0]
    }

    pub(crate) fn m11(&self) -> f64 {
        self.underlying[1][1]
    }

    fn m12(&self) -> f64 {
        self.underlying[1][2]
    }

    fn m20(&self) -> f64 {
        self.underlying[2][0]
    }

    fn m21(&self) -> f64 {
        self.underlying[2][1]
    }

    pub(crate) fn m22(&self) -> f64 {
        self.underlying[2][2]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(in crate::core) struct Matrix2D {
    underlying: [[f64; 2]; 2],
}

impl Matrix2D {
    pub fn new(row0: [f64; 2], row1: [f64; 2]) -> Self {
        Matrix2D {
            underlying: [row0, row1],
        }
    }

    pub(crate) fn determinant(&self) -> f64 {
        self.m00() * self.m11() - self.m01() * self.m10()
    }

    pub(crate) fn m00(&self) -> f64 {
        self.underlying[0][0]
    }

    pub(crate) fn m01(&self) -> f64 {
        self.underlying[0][1]
    }

    pub(crate) fn m10(&self) -> f64 {
        self.underlying[1][0]
    }

    pub(crate) fn m11(&self) -> f64 {
        self.underlying[1][1]
    }
}

impl Matrix4D {
    pub fn m00(&self) -> f64 {
        self.underlying[0][0]
    }

    pub fn m01(&self) -> f64 {
        self.underlying[0][1]
    }

    pub fn m02(&self) -> f64 {
        self.underlying[0][2]
    }

    pub fn m03(&self) -> f64 {
        self.underlying[0][3]
    }

    pub fn m10(&self) -> f64 {
        self.underlying[1][0]
    }

    pub fn m11(&self) -> f64 {
        self.underlying[1][1]
    }

    pub fn m12(&self) -> f64 {
        self.underlying[1][2]
    }

    pub fn m13(&self) -> f64 {
        self.underlying[1][3]
    }

    pub fn m20(&self) -> f64 {
        self.underlying[2][0]
    }

    pub fn m21(&self) -> f64 {
        self.underlying[2][1]
    }

    pub fn m22(&self) -> f64 {
        self.underlying[2][2]
    }

    pub fn m23(&self) -> f64 {
        self.underlying[2][3]
    }

    pub fn m30(&self) -> f64 {
        self.underlying[3][0]
    }

    pub fn m31(&self) -> f64 {
        self.underlying[3][1]
    }

    pub fn m32(&self) -> f64 {
        self.underlying[3][2]
    }

    pub fn m33(&self) -> f64 {
        self.underlying[3][3]
    }
}

impl AbsDiffEq for Matrix4D {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON as f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.m00().abs_diff_eq(&other.m00(), epsilon)
            && self.m01().abs_diff_eq(&other.m01(), epsilon)
            && self.m02().abs_diff_eq(&other.m02(), epsilon)
            && self.m03().abs_diff_eq(&other.m03(), epsilon)
            && self.m10().abs_diff_eq(&other.m10(), epsilon)
            && self.m11().abs_diff_eq(&other.m11(), epsilon)
            && self.m12().abs_diff_eq(&other.m12(), epsilon)
            && self.m13().abs_diff_eq(&other.m13(), epsilon)
            && self.m20().abs_diff_eq(&other.m20(), epsilon)
            && self.m21().abs_diff_eq(&other.m21(), epsilon)
            && self.m22().abs_diff_eq(&other.m22(), epsilon)
            && self.m23().abs_diff_eq(&other.m23(), epsilon)
            && self.m30().abs_diff_eq(&other.m30(), epsilon)
            && self.m31().abs_diff_eq(&other.m31(), epsilon)
            && self.m32().abs_diff_eq(&other.m32(), epsilon)
            && self.m33().abs_diff_eq(&other.m33(), epsilon)
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use crate::util::ReasonableF64;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Matrix4D {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut gen_row = || {
                [
                    ReasonableF64::arbitrary(g).0,
                    ReasonableF64::arbitrary(g).0,
                    ReasonableF64::arbitrary(g).0,
                    ReasonableF64::arbitrary(g).0,
                ]
            };

            Matrix4D::new(gen_row(), gen_row(), gen_row(), gen_row())
        }
    }
}
