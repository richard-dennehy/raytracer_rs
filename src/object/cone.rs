use crate::object::bounds::BoundingBox;
use crate::object::Shape;
use crate::{Intersection, Normal3D, Object, Point3D, Ray, Vector, Vector3D};
use std::f64::consts::PI;

/// An infinite double-napped cone (like a sand timer), tapering to a point at the origin,
/// centred on the y axis, with a radius equal to the absolute y value (i.e. the radius is 1 at y -1)
///
/// May be truncated at either end, to make the shape finite. Truncating at y = 0 produces a single cone.
/// May be capped, otherwise the end will be open and the inner face will be visible
#[derive(Debug, PartialEq)]
pub struct Cone {
    max_y: f64,
    min_y: f64,
    capped: bool,
}

impl Shape for Cone {
    fn object_bounds(&self) -> BoundingBox {
        let limit = self.min_y.abs().max(self.max_y.abs());

        BoundingBox::new(
            Point3D::new(-limit, self.min_y, -limit),
            Point3D::new(limit, self.max_y, limit),
        )
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        let distance = point.x().powi(2) + point.z().powi(2);

        if distance < point.y() && point.y() >= self.max_y - f64::EPSILON {
            Normal3D::POSITIVE_Y
        } else if distance < point.y() && point.y() <= self.min_y + f64::EPSILON {
            Normal3D::NEGATIVE_Y
        } else {
            let y = distance.sqrt();

            if point.y() > 0.0 {
                Vector3D::new(point.x(), -y, point.z())
            } else {
                Vector3D::new(point.x(), y, point.z())
            }
            .normalised()
        }
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        ray: Ray,
    ) -> Vec<Intersection<'parent>> {
        let intersects_cap = |t: f64| {
            let x = ray.origin.x() + t * ray.direction.x();
            let y = ray.origin.y() + t * ray.direction.y();
            let z = ray.origin.z() + t * ray.direction.z();

            (x.powi(2) + z.powi(2)) <= y.abs()
        };

        let mut cap_intersections = if self.capped {
            let mut ts = Vec::with_capacity(2);
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
            vec![]
        };

        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);
        let b = 2.0 * ray.origin.x() * ray.direction.x() - 2.0 * ray.origin.y() * ray.direction.y()
            + 2.0 * ray.origin.z() * ray.direction.z();

        let c = ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);

        if a.abs() <= f64::EPSILON && b.abs() <= f64::EPSILON {
            return cap_intersections;
        };

        if a.abs() <= f64::EPSILON {
            let t = -c / (2.0 * b);
            cap_intersections.push(Intersection::new(t, parent));
            return cap_intersections;
        };

        let mut ts = if let Some((first, second)) = crate::util::quadratic(a, b, c) {
            let mut ts = Vec::with_capacity(2);

            let y_first = ray.origin.y() + ray.direction.y() * first;
            if y_first > self.min_y && y_first < self.max_y {
                ts.push(Intersection::new(first, parent));
            }

            let y_second = ray.origin.y() + ray.direction.y() * second;
            if y_second > self.min_y && y_second < self.max_y {
                ts.push(Intersection::new(second, parent));
            }

            ts
        } else {
            vec![]
        };

        ts.append(&mut cap_intersections);

        ts
    }

    /// ranges from u <- 0..3 and v <- 0..1 such that:
    ///  - u <- 0..1 maps to the sides of the cone,
    ///  - u <- 1..2 maps to the top cap of the cone
    ///  - u <- 2..3 maps to the bottom cap of the cone
    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        if self.capped && ((self.max_y - point.y()).abs() <= f32::EPSILON as f64) {
            let u = (point.x() + 1.0) / 2.0;
            let v = (1.0 - point.z()) / 2.0;

            return (u + 1.0, v);
        }

        if self.capped && ((self.min_y - point.y()).abs() <= f32::EPSILON as f64) {
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

pub struct ConeBuilder {
    min_y: f64,
    max_y: f64,
    capped: bool,
}

impl ConeBuilder {
    pub(super) fn new() -> Self {
        ConeBuilder {
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
        Object::from_shape(Box::new(Cone {
            min_y: self.min_y,
            max_y: self.max_y,
            capped: self.capped,
        }))
    }
}
