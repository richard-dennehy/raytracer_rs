use crate::object::bounds::BoundingBox;
use crate::object::Shape;
use crate::{Intersection, Intersections, Normal3D, Object, Point3D, Ray, Vector, Vector3D};

#[derive(Debug, PartialEq)]
pub struct Triangle {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    edge1: Vector3D,
    edge2: Vector3D,
    kind: NormalKind,
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
            kind: NormalKind::Uniform(normal),
        }
    }

    pub fn smooth(
        point1: Point3D,
        point2: Point3D,
        point3: Point3D,
        normal1: Normal3D,
        normal2: Normal3D,
        normal3: Normal3D,
    ) -> Self {
        let edge1 = point2 - point1;
        let edge2 = point3 - point1;

        Triangle {
            p1: point1,
            p2: point2,
            p3: point3,
            edge1,
            edge2,
            kind: NormalKind::Smooth {
                normal1,
                normal2,
                normal3,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum NormalKind {
    Smooth {
        normal1: Normal3D,
        normal2: Normal3D,
        normal3: Normal3D,
    },
    Uniform(Normal3D),
}

impl Shape for Triangle {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(
            Point3D::min([self.p1, self.p2, self.p3]),
            Point3D::max([self.p1, self.p2, self.p3]),
        )
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        match self.kind {
            NormalKind::Smooth {
                normal1,
                normal2,
                normal3,
            } => {
                let (u, v) = self.uv_at(point);
                (normal2 * u + normal3 * v + normal1 * (1.0 - u - v)).normalised()
            }
            NormalKind::Uniform(normal) => normal,
        }
    }

    /// Möller–Trumbore algorithm
    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Intersections<'parent> {
        let dir_cross_e2 = with.direction.cross(self.edge2);
        let determinant = self.edge1.dot(dir_cross_e2);

        if determinant.abs() < f64::EPSILON {
            return Intersections::empty();
        };

        let f = 1.0 / determinant;
        let p1_to_origin = with.origin - self.p1;

        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return Intersections::empty();
        };

        let origin_cross_e1 = p1_to_origin.cross(self.edge1);
        let v = f * with.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return Intersections::empty();
        };

        let t = f * self.edge2.dot(origin_cross_e1);
        // it'd be nice to not throw away the UV here, but it doesn't appear to be possible without
        // compromising the performance of every other kind of shape
        Intersections::single(Intersection::new(t, parent))
    }

    // calculate Barycentric coordinates; see https://en.wikipedia.org/wiki/Barycentric_coordinate_system#Barycentric_coordinates_on_triangles
    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        let edge3 = self.p3 - self.p1;
        let point_to_origin = point - self.p1;

        let e1_dot_e1 = self.edge1.dot(self.edge1);
        let e1_dot_e3 = self.edge1.dot(edge3);
        let e3_dot_e3 = edge3.dot(edge3);
        // TODO this should probably be pre-computed (inverted)
        let denominator = e1_dot_e1 * e3_dot_e3 - e1_dot_e3.powi(2);

        let point_dot_e1 = point_to_origin.dot(self.edge1);
        let point_dot_e3 = point_to_origin.dot(edge3);

        let v = (e3_dot_e3 * point_dot_e1 - e1_dot_e3 * point_dot_e3) / denominator;
        let w = (e1_dot_e1 * point_dot_e3 - e1_dot_e3 * point_dot_e1) / denominator;

        // using `v` and `w` like this (and ignoring `u`) gives the same coordinates as Möller–Trumbore
        (v, w)
    }
}
