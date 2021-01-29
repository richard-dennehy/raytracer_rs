use crate::object::Shape;
use crate::{Intersection, Object, Point3D, Ray, Vector3D};

#[derive(Debug, PartialEq)]
pub struct Cone {
    max_y: f64,
    min_y: f64,
    capped: bool,
}
impl Shape for Cone {
    fn object_normal_at(&self, point: Point3D) -> Vector3D {
        let distance = point.x().powi(2) + point.z().powi(2);

        if distance < point.y() && point.y() >= self.max_y - f64::EPSILON {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if distance < point.y() && point.y() <= self.min_y + f64::EPSILON {
            Vector3D::new(0.0, -1.0, 0.0)
        } else {
            let y = distance.sqrt();

            if point.y() > 0.0 {
                Vector3D::new(point.x(), -y, point.z())
            } else {
                Vector3D::new(point.x(), y, point.z())
            }
        }
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        let intersects_cap = |t: f64| {
            let x = with.origin.x() + t * with.direction.x();
            let y = with.origin.y() + t * with.direction.y();
            let z = with.origin.z() + t * with.direction.z();

            (x.powi(2) + z.powi(2)) <= y.abs()
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

        let a =
            with.direction.x().powi(2) - with.direction.y().powi(2) + with.direction.z().powi(2);
        let b = 2.0 * with.origin.x() * with.direction.x()
            - 2.0 * with.origin.y() * with.direction.y()
            + 2.0 * with.origin.z() * with.direction.z();

        let c = with.origin.x().powi(2) - with.origin.y().powi(2) + with.origin.z().powi(2);

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

            let y_first = with.origin.y() + with.direction.y() * first;
            if y_first > self.min_y && y_first < self.max_y {
                ts.push(Intersection::new(first, parent));
            }

            let y_second = with.origin.y() + with.direction.y() * second;
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

    #[cfg(test)]
    fn vertices(&self) -> Vec<Point3D> {
        unimplemented!()
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
        Object::shape(Box::new(Cone {
            min_y: self.min_y,
            max_y: self.max_y,
            capped: self.capped,
        }))
    }
}