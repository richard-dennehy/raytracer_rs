use crate::core::Ray;
use crate::core::{Normal3D, Point3D, Vector3D, VectorMaths};
use crate::scene::bounding_box::BoundingBox;
use crate::scene::{Intersection, Shape};
use crate::scene::{Intersections, Object};

#[derive(Debug, PartialEq)]
pub struct Triangle {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    edge1: Vector3D,
    edge2: Vector3D,
    denominator: f64,
    kind: NormalKind,
}

impl Triangle {
    pub fn new(point1: Point3D, point2: Point3D, point3: Point3D) -> Self {
        let edge1 = point2 - point1;
        let edge2 = point3 - point1;

        let normal = (edge2.cross(edge1)).normalised();
        let denominator = 1.0 / (edge1.dot(edge1) * edge2.dot(edge2) - edge1.dot(edge2).powi(2));

        Triangle {
            p1: point1,
            p2: point2,
            p3: point3,
            edge1,
            edge2,
            denominator,
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
        let denominator = 1.0 / (edge1.dot(edge1) * edge2.dot(edge2) - edge1.dot(edge2).powi(2));

        Triangle {
            p1: point1,
            p2: point2,
            p3: point3,
            edge1,
            edge2,
            denominator,
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
        let point_to_origin = point - self.p1;

        let e1_dot_e1 = self.edge1.dot(self.edge1);
        let e1_dot_e2 = self.edge1.dot(self.edge2);
        let e2_dot_e2 = self.edge2.dot(self.edge2);

        let point_dot_e1 = point_to_origin.dot(self.edge1);
        let point_dot_e2 = point_to_origin.dot(self.edge2);

        let v = (e2_dot_e2 * point_dot_e1 - e1_dot_e2 * point_dot_e2) * self.denominator;
        let w = (e1_dot_e1 * point_dot_e2 - e1_dot_e2 * point_dot_e1) * self.denominator;

        // using `v` and `w` like this (and ignoring `u`) gives the same coordinates as Möller–Trumbore
        (v, w)
    }
}
