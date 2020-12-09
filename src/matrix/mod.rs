#[cfg(test)]
mod tests;

pub struct Matrix4D {
    underlying: [[f64; 4]; 4],
}

impl Matrix4D {
    pub fn new(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Self {
        Matrix4D {
            underlying: [row0, row1, row2, row3],
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

pub struct Matrix3D {
    underlying: [[f64; 3]; 3],
}

impl Matrix3D {
    pub fn new(row0: [f64; 3], row1: [f64; 3], row2: [f64; 3]) -> Self {
        Matrix3D {
            underlying: [row0, row1, row2],
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

pub struct Matrix2D {
    underlying: [[f64; 2]; 2],
}

impl Matrix2D {
    pub fn new(row0: [f64; 2], row1: [f64; 2]) -> Self {
        Matrix2D {
            underlying: [row0, row1],
        }
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
