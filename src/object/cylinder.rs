use crate::object::Shape;
use crate::{Intersection, Object, Point3D, Ray, Vector3D};

#[derive(Debug, PartialEq)]
pub struct Cylinder {
    max_y: f64,
    min_y: f64,
    capped: bool,
}
impl Shape for Cylinder {
    fn object_normal_at(&self, point: Point3D, _uv: Option<(f64, f64)>) -> Vector3D {
        let distance = point.x().powi(2) + point.z().powi(2);

        if distance < 1.0 && point.y() >= self.max_y - f64::EPSILON {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if distance < 1.0 && point.y() <= self.min_y + f64::EPSILON {
            Vector3D::new(0.0, -1.0, 0.0)
        } else {
            Vector3D::new(point.x(), 0.0, point.z())
        }
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        let intersects_cap = |t: f64| {
            let x = with.origin.x() + t * with.direction.x();
            let z = with.origin.z() + t * with.direction.z();

            (x.powi(2) + z.powi(2)) <= 1.0
        };

        let mut cap_intersections = if self.capped {
            let mut ts = Vec::with_capacity(2);
            // check bottom cap
            let t = (self.min_y - with.origin.y()) / with.direction.y();

            if intersects_cap(t) {
                ts.push(Intersection::new(t, parent));
            }

            // check top cap
            let t = (self.max_y - with.origin.y()) / with.direction.y();

            if intersects_cap(t) {
                ts.push(Intersection::new(t, parent));
            }

            ts
        } else {
            vec![]
        };

        let a = with.direction.x().powi(2) + with.direction.z().powi(2);

        if a.abs() <= f64::EPSILON {
            return cap_intersections;
        };

        let b =
            2.0 * with.origin.x() * with.direction.x() + 2.0 * with.origin.z() * with.direction.z();
        let c = with.origin.x().powi(2) + with.origin.z().powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return cap_intersections;
        };

        let first = (-b - discriminant.sqrt()) / (2.0 * a);
        let second = (-b + discriminant.sqrt()) / (2.0 * a);

        let y_first = with.origin.y() + with.direction.y() * first;
        let y_second = with.origin.y() + with.direction.y() * second;

        let mut ts = Vec::with_capacity(2);
        if y_first > self.min_y && y_first < self.max_y {
            ts.push(Intersection::new(first, parent));
        }

        if y_second > self.min_y && y_second < self.max_y {
            ts.push(Intersection::new(second, parent));
        }

        ts.append(&mut cap_intersections);

        ts
    }
}

pub struct CylinderBuilder {
    min_y: f64,
    max_y: f64,
    capped: bool,
}

impl CylinderBuilder {
    pub(super) fn new() -> Self {
        CylinderBuilder {
            min_y: -f64::INFINITY,
            max_y: f64::INFINITY,
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
