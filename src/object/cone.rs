use crate::object::Shape;
use crate::{Intersection, Normal3D, Object, Point3D, Ray, Vector, Vector3D};

#[derive(Debug, PartialEq)]
pub struct Cone {
    max_y: f64,
    min_y: f64,
    capped: bool,
}
impl Shape for Cone {
    fn object_normal_at(&self, point: Point3D, _uv: Option<(f64, f64)>) -> Normal3D {
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
        Object::from_shape(Box::new(Cone {
            min_y: self.min_y,
            max_y: self.max_y,
            capped: self.capped,
        }))
    }
}
