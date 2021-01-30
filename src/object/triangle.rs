use crate::object::Shape;
use crate::{Intersection, Object, Point3D, Ray, Vector3D};

#[derive(Debug)]
pub struct Triangle {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    edge1: Vector3D,
    edge2: Vector3D,
    normal: Vector3D,
}

impl Triangle {
    pub fn new(point1: Point3D, point2: Point3D, point3: Point3D) -> Self {
        let edge1 = point2 - point1;
        let edge2 = point3 - point2;

        let normal = (edge2.cross(edge1)).normalised();

        Triangle {
            p1: point1,
            p2: point2,
            p3: point3,
            edge1,
            edge2,
            normal,
        }
    }
}

impl Shape for Triangle {
    fn object_normal_at(&self, _: Point3D, _uv: Option<(f64, f64)>) -> Vector3D {
        self.normal
    }

    /// Möller–Trumbore algorithm
    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        let dir_cross_e2 = with.direction.cross(self.edge2);
        let determinant = self.edge1.dot(dir_cross_e2);

        if determinant.abs() < f64::EPSILON {
            return vec![];
        };

        let f = 1.0 / determinant;
        let p1_to_origin = with.origin - self.p1;

        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        };

        let origin_cross_e1 = p1_to_origin.cross(self.edge1);
        let v = f * with.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        };

        let t = f * self.edge2.dot(origin_cross_e1);
        vec![Intersection::with_uv(t, parent, u, v)]
    }

    #[cfg(test)]
    fn vertices(&self) -> Vec<Point3D> {
        vec![self.p1, self.p2, self.p3]
    }
}
