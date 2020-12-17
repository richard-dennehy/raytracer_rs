use crate::Matrix4D;

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

    pub fn transform(&mut self, transform: Matrix4D) {
        self.transform = transform
    }
}
