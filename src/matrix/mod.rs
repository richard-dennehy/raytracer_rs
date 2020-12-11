use crate::{Point3D, Vector3D};
use std::ops::Mul;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix4D {
    underlying: [[f64; 4]; 4],
}

impl Matrix4D {
    pub const fn new(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Self {
        Matrix4D {
            underlying: [row0, row1, row2, row3],
        }
    }

    pub const fn identity() -> Self {
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

    fn determinant(&self) -> f64 {
        self.m00() * self.cofactor(0, 0)
            + self.m01() * self.cofactor(0, 1)
            + self.m02() * self.cofactor(0, 2)
            + self.m03() * self.cofactor(0, 3)
    }

    fn cofactor(&self, row: u8, column: u8) -> f64 {
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

    pub fn submatrix(&self, excluding_row: u8, excluding_column: u8) -> Matrix3D {
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
                [self.m00(), self.m01(), self.m03()],
                [self.m10(), self.m11(), self.m13()],
                [self.m30(), self.m31(), self.m33()],
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

    fn mul(self, rhs: Matrix4D) -> Self::Output {
        Matrix4D::new(
            [
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
            ],
            [
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
            ],
            [
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
            ],
            [
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
            ],
        )
    }
}

impl Mul<Vector3D> for Matrix4D {
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

impl Mul<Point3D> for Matrix4D {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix3D {
    underlying: [[f64; 3]; 3],
}

impl Matrix3D {
    pub fn new(row0: [f64; 3], row1: [f64; 3], row2: [f64; 3]) -> Self {
        Matrix3D {
            underlying: [row0, row1, row2],
        }
    }

    fn determinant(&self) -> f64 {
        self.m00() * self.cofactor(0, 0)
            + self.m01() * self.cofactor(0, 1)
            + self.m02() * self.cofactor(0, 2)
    }

    fn cofactor(&self, row: u8, column: u8) -> f64 {
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

    fn submatrix(&self, excluding_row: u8, excluding_column: u8) -> Matrix2D {
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

    pub fn m00(&self) -> f64 {
        self.underlying[0][0]
    }

    pub fn m01(&self) -> f64 {
        self.underlying[0][1]
    }

    pub fn m02(&self) -> f64 {
        self.underlying[0][2]
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

    pub fn m20(&self) -> f64 {
        self.underlying[2][0]
    }

    pub fn m21(&self) -> f64 {
        self.underlying[2][1]
    }

    pub fn m22(&self) -> f64 {
        self.underlying[2][2]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix2D {
    underlying: [[f64; 2]; 2],
}

impl Matrix2D {
    pub fn new(row0: [f64; 2], row1: [f64; 2]) -> Self {
        Matrix2D {
            underlying: [row0, row1],
        }
    }

    pub fn determinant(&self) -> f64 {
        self.m00() * self.m11() - self.m01() * self.m10()
    }

    pub fn m00(&self) -> f64 {
        self.underlying[0][0]
    }

    pub fn m01(&self) -> f64 {
        self.underlying[0][1]
    }

    pub fn m10(&self) -> f64 {
        self.underlying[1][0]
    }

    pub fn m11(&self) -> f64 {
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
