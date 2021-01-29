use crate::object::Shape;
use crate::{Intersection, Object, Point3D, Ray, Vector3D};

#[derive(Debug)]
pub struct SmoothTriangle {
    point1: Point3D,
    point2: Point3D,
    point3: Point3D,
    normal1: Vector3D,
    normal2: Vector3D,
    normal3: Vector3D,
}

impl SmoothTriangle {
    pub fn new(
        point1: Point3D,
        point2: Point3D,
        point3: Point3D,
        normal1: Vector3D,
        normal2: Vector3D,
        normal3: Vector3D,
    ) -> Self {
        SmoothTriangle {
            point1,
            point2,
            point3,
            normal1,
            normal2,
            normal3,
        }
    }
}

impl Shape for SmoothTriangle {
    fn object_normal_at(&self, point: Point3D) -> Vector3D {
        unimplemented!()
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        unimplemented!()
    }

    #[cfg(test)]
    fn vertices(&self) -> Vec<Point3D> {
        unimplemented!()
    }
}
