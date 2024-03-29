use crate::core::F64Ext;
use crate::core::Ray;
use crate::core::{Normal3D, Point3D, Vector3D, VectorMaths};
use crate::scene::bounding_box::BoundingBox;
use crate::scene::intersection::{Intersection, Intersections};
use crate::scene::Object;
use crate::scene::Shape;
use std::f64::consts::PI;

/// An infinite cylinder centred on the y axis, with a constant radius of 1
///
/// May be truncated at either end to make it finite.
/// May be capped, otherwise the ends will be open, and the inner face will be visible
#[derive(Debug, PartialEq)]
pub struct Cylinder {
    max_y: f64,
    min_y: f64,
    capped: bool,
}

impl Shape for Cylinder {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(
            Point3D::new(-1.0, self.min_y, -1.0),
            Point3D::new(1.0, self.max_y, 1.0),
        )
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        if self.capped && point.y().is_roughly_gte(self.max_y) {
            Normal3D::POSITIVE_Y
        } else if self.capped && point.y().is_roughly_lte(self.min_y) {
            Normal3D::NEGATIVE_Y
        } else {
            Vector3D::new(point.x(), 0.0, point.z()).normalised()
        }
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        ray: Ray,
    ) -> Intersections<'parent> {
        let intersects_cap = |t: f64| {
            let x = ray.origin.x() + t * ray.direction.x();
            let z = ray.origin.z() + t * ray.direction.z();

            (x.powi(2) + z.powi(2)).is_roughly_lte(1.0)
        };

        let cap_intersections = if self.capped {
            let mut ts = Intersections::empty();
            // check bottom cap
            let t = (self.min_y - ray.origin.y()) / ray.direction.y();

            if intersects_cap(t) {
                ts.push(Intersection::new(t, parent));
            }

            // check top cap
            let t = (self.max_y - ray.origin.y()) / ray.direction.y();

            if intersects_cap(t) {
                ts.push(Intersection::new(t, parent));
            }

            ts
        } else {
            Intersections::empty()
        };

        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

        if a.abs() <= f64::EPSILON {
            return cap_intersections;
        };

        let b = 2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();
        let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return cap_intersections;
        };

        let first = (-b - discriminant.sqrt()) / (2.0 * a);
        let second = (-b + discriminant.sqrt()) / (2.0 * a);

        let y_first = ray.origin.y() + ray.direction.y() * first;
        let y_second = ray.origin.y() + ray.direction.y() * second;

        let mut ts = Intersections::empty();
        if y_first > self.min_y && y_first < self.max_y {
            ts.push(Intersection::new(first, parent));
        }

        if y_second > self.min_y && y_second < self.max_y {
            ts.push(Intersection::new(second, parent));
        }

        ts.join(cap_intersections)
    }

    /// ranges from u <- 0..3 and v <- 0..1 such that:
    ///  - u <- 0..1 maps to the sides of the cylinder
    ///  - u <- 1..2 maps to the top cap of the cylinder
    ///  - u <- 2..3 maps to the bottom cap of the cylinder
    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        if self.capped && self.max_y.roughly_equals(point.y()) {
            let u = (point.x() + 1.0) / 2.0;
            let v = (1.0 - point.z()) / 2.0;

            return (u + 1.0, v);
        }

        if self.capped && self.min_y.roughly_equals(point.y()) {
            let u = (point.x() + 1.0) / 2.0;
            let v = (point.z() + 1.0) / 2.0;

            return (u + 2.0, v);
        }

        // azimuthal angle
        let theta = point.x().atan2(point.z());
        let raw_u = theta / (2.0 * PI);
        // corrects backwards azimuthal angle
        let u = 1.0 - (raw_u + 0.5);

        let v = point.y().rem_euclid(1.0);
        (u, v)
    }
}

pub struct CylinderBuilder {
    min_y: f64,
    max_y: f64,
    capped: bool,
}

impl CylinderBuilder {
    pub(in crate::scene) fn new() -> Self {
        CylinderBuilder {
            min_y: -f64::MAX,
            max_y: f64::MAX,
            capped: false,
        }
    }

    pub fn min_y(mut self, min_y: f64) -> Self {
        self.min_y = min_y;
        self
    }

    pub fn max_y(mut self, max_y: f64) -> Self {
        self.max_y = max_y;
        self
    }

    pub fn capped(mut self) -> Self {
        self.capped = true;
        self
    }

    pub fn build(self) -> Object {
        Object::from_shape(Box::new(Cylinder {
            min_y: self.min_y,
            max_y: self.max_y,
            capped: self.capped,
        }))
    }
}
