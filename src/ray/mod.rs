use crate::{Point3D, Sphere, Vector3D};
use std::cmp::Ordering;

#[cfg(test)]
mod tests;

pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point3D {
        self.origin + self.direction * time
    }

    pub fn intersect<'with>(&self, with: &'with Sphere) -> Option<Intersection<'with>> {
        let sphere_to_ray = self.origin - with.origin();
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0; // <- Radius?

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some(Intersection::new(t1, t2, with))
    }
}

#[derive(Debug)]
pub struct Intersection<'with> {
    pub first: f64,
    pub second: f64,
    pub with: &'with Sphere,
}

impl<'with> Intersection<'with> {
    pub fn new(first: f64, second: f64, with: &'with Sphere) -> Intersection {
        debug_assert!(first <= second, "the first `t` value should always be less than (or equal to) the second `t` value - Intersections::hit relies on this invariant");

        Intersection {
            first,
            second,
            with,
        }
    }
}

pub struct Intersections<'scene>(Vec<Intersection<'scene>>);

/// Invariant: always non-empty
impl<'scene> Intersections<'scene> {
    pub fn of(intersection: Intersection<'scene>) -> Self {
        Intersections(vec![intersection])
    }

    pub fn push(mut self, intersection: Intersection<'scene>) -> Self {
        self.0.push(intersection);

        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<Hit<'scene>> {
        self.0
            .iter()
            .max_by(|&i1, &i2| {
                if i1.second > i2.second {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .map(|intersect| Hit::new(intersect.second, intersect.with))
    }
}

pub struct Hit<'object> {
    pub t: f64,
    pub object: &'object Sphere,
}

impl<'object> Hit<'object> {
    pub fn new(t: f64, object: &'object Sphere) -> Self {
        Hit { t, object }
    }
}
