use crate::core::{Normal3D, Point3D, Ray, Vector, Vector3D};
use crate::scene::bounding_box::BoundingBox;
use crate::scene::intersection::{Intersection, Intersections};
use crate::scene::shape::Shape;
use crate::scene::Object;

#[derive(Debug, PartialEq)]
// a 2x2x2 cube, centred at the world Origin (i.e. from (-1, -1, -1) to (1, 1, 1))
pub struct Cube;
impl Shape for Cube {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0))
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        if point.x().abs() >= point.y().abs() && point.x().abs() >= point.z().abs() {
            Vector3D::new(point.x(), 0.0, 0.0)
        } else if point.y().abs() >= point.x().abs() && point.y().abs() >= point.z().abs() {
            Vector3D::new(0.0, point.y(), 0.0)
        } else {
            Vector3D::new(0.0, 0.0, point.z())
        }
        .normalised()
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Intersections<'parent> {
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let t_min_numerator = -1.0 - origin;
            let t_max_numerator = 1.0 - origin;

            let t_min = t_min_numerator / direction;
            let t_max = t_max_numerator / direction;

            if t_min > t_max {
                (t_max, t_min)
            } else {
                (t_min, t_max)
            }
        }

        let (t_min_x, t_max_x) = check_axis(with.origin.x(), with.direction.x());
        let (t_min_y, t_max_y) = check_axis(with.origin.y(), with.direction.y());
        let (t_min_z, t_max_z) = check_axis(with.origin.z(), with.direction.z());

        let t_min = t_min_x.max(t_min_y).max(t_min_z);
        let t_max = t_max_x.min(t_max_y).min(t_max_z);

        if t_min > t_max {
            Intersections::empty()
        } else {
            Intersections::pair(
                Intersection::new(t_min, parent),
                Intersection::new(t_max, parent),
            )
        }
    }

    /// ranges from u <- 0..3 and v <- 0..4 such that:
    ///  - u <- 1..2; v <- 0..1 maps to the top face
    ///  - u <- 1..2; v <- 1..2 maps to the right face
    ///  - u <- 0..1; v <- 2..3 maps to the front face
    ///  - u <- 1..2; v <- 2..3 maps to the bottom face
    ///  - u <- 2..3; v <- 2..3 maps to the back face
    ///  - u <- 1..2; v <- 3..4 maps to the left face
    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        let largest = point.x().abs().max(point.y().abs().max(point.z().abs()));

        if largest == point.x() {
            // right face
            let u = (1.0 - point.z()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 1.0)
        } else if largest == -point.x() {
            // left face
            let u = (1.0 + point.z()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 3.0)
        } else if largest == point.y() {
            // top face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 - point.z()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v)
        } else if largest == -point.y() {
            // bottom face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.z()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 2.0)
        } else if largest == point.z() {
            // front face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u, v + 2.0)
        } else {
            // back face
            let u = (1.0 - point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 2.0, v + 2.0)
        }
    }
}
