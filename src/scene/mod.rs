mod object;
pub use object::Object;

mod bounding_box;
use bounding_box::BoundingBox;

mod light;
pub use light::{Light, LightSample};

mod material;
pub use material::{Material, MaterialKind};

mod pattern;
pub use pattern::{Pattern, UvPattern};

mod world;
pub use world::{World, WorldSettings};

mod intersection;
pub use intersection::{HitData, Intersection, Intersections, ReflectionData};

pub use shape::{cone::ConeBuilder, cylinder::CylinderBuilder};
use shape::{cube::Cube, plane::Plane, sphere::Sphere, triangle::Triangle, Shape};
mod shape {
    use super::*;
    use crate::core::{Normal3D, Point3D, Ray};
    use std::fmt::Debug;

    pub trait Shape: Debug + Sync {
        fn object_bounds(&self) -> BoundingBox;

        fn object_normal_at(&self, point: Point3D) -> Normal3D;
        fn object_intersect<'parent>(
            &self,
            parent: &'parent Object,
            with: Ray,
        ) -> Intersections<'parent>;

        fn uv_at(&self, point: Point3D) -> (f64, f64);
    }

    pub mod cone;
    pub mod cube;
    pub mod cylinder;
    pub mod plane;
    pub mod sphere;
    pub mod triangle;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bounding_box_tests;
    mod cone_tests;
    mod cube_tests;
    mod cylinder_tests;
    mod intersection_tests;
    mod object_tests;
    mod pattern_tests;
    mod plane_tests;
    mod sphere_tests;
    mod triangle_tests;
    mod world_tests;
}
