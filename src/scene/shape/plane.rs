use crate::core::{F64Ext, Normal3D, Point3D, Ray, Vector};
use crate::scene::bounding_box::BoundingBox;
use crate::scene::intersection::{Intersection, Intersections};
use crate::scene::shape::Shape;
use crate::scene::Object;

#[derive(Debug, PartialEq)]
// an infinite XZ plane
pub struct Plane;
impl Shape for Plane {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(
            Point3D::new(-f64::MAX, 0.0, -f64::MAX),
            Point3D::new(f64::MAX, 0.0, f64::MAX),
        )
    }

    fn object_normal_at(&self, _: Point3D) -> Normal3D {
        Normal3D::POSITIVE_Y
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Intersections<'parent> {
        if with.direction.y().is_roughly_zero() {
            return Intersections::empty();
        }

        let t = -with.origin.y() / with.direction.y();
        Intersections::single(Intersection::new(t, parent))
    }

    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        (point.x().rem_euclid(1.0), point.z().rem_euclid(1.0))
    }
}
