use crate::{Matrix4D, Point3D, Vector3D};

#[cfg(test)]
mod tests;

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

    pub fn normal_at(&self, point: Point3D) -> Vector3D {
        let inverted_transform = self
            .transform
            .inverse()
            .expect("transformation Matrix must be invertible");

        let (x, y, z, w) = &inverted_transform * point;

        debug_assert!(w == 1.0, "Point transformation did not return a point");
        let object_point = Point3D::new(x, y, z);
        let object_normal = object_point - Point3D::new(0.0, 0.0, 0.0); // sphere origin

        // deliberately ignoring `w` as a translation Matrix may affect `w` so it's no longer 0
        let (x, y, z, _) = inverted_transform.transpose() * object_normal;
        let world_normal = Vector3D::new(x, y, z);
        world_normal.normalised()
    }
}
