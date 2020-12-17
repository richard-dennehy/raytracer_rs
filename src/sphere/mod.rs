use crate::{Matrix4D, Point3D};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4D,
}

impl Sphere {
    pub const fn unit() -> Self {
        Sphere {
            transform: Matrix4D::identity(),
        }
    }

    pub fn origin(&self) -> Point3D {
        Point3D::new(0.0, 0.0, 0.0)
    }

    pub fn radius(&self) -> f64 {
        1.0
    }

    pub fn transform(&mut self, transform: Matrix4D) {
        self.transform = transform
    }
}
