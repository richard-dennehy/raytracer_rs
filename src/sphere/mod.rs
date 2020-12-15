use crate::{Matrix4D, Point3D};

pub struct Sphere {
    transform: Matrix4D,
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
}
