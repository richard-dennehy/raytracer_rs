use crate::core::{Intersection, Intersections, Normal3D, Point3D, Ray, Vector};
use crate::scene::bounding_box::BoundingBox;
use crate::scene::shape::Shape;
use crate::scene::Object;
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
/// A unit sphere, with the centre at the world origin, and a radius of 1
pub struct Sphere;
impl Shape for Sphere {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0))
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        (point - Point3D::ORIGIN).normalised()
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Intersections<'parent> {
        let sphere_to_ray = with.origin - Point3D::ORIGIN;
        let a = with.direction.dot(with.direction);
        let b = 2.0 * with.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        if let Some((first, second)) = crate::core::quadratic(a, b, c) {
            Intersections::pair(
                Intersection::new(first, parent),
                Intersection::new(second, parent),
            )
        } else {
            Intersections::empty()
        }
    }

    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        // See https://en.wikipedia.org/wiki/Spherical_coordinate_system noting this uses _mathematical_ notation

        // azimuthal angle - this is backwards but gets corrected later
        let theta = point.x().atan2(point.z());
        // given the centre is at the world origin, the radius is given by the magnitude of the vector
        // from the world origin to the point
        let r = (point - Point3D::ORIGIN).magnitude();
        // polar angle
        let phi = (point.y() / r).acos();
        let raw_u = theta / (2.0 * PI);
        // corrects backwards azimuthal angle
        let u = 1.0 - (raw_u + 0.5);
        // subtract from 1 to invert `v` such that 1 is the northernmost point
        let v = 1.0 - (phi / PI);

        (u, v)
    }
}
