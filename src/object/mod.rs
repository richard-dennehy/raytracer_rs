use crate::{
    Colour, Intersection, Intersections, Light, Material, Matrix4D, Point3D, Ray, Vector3D,
};
use std::fmt::Debug;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(test)]
mod tests;

mod cylinder;
pub use cylinder::CylinderBuilder;

mod cone;
pub use cone::ConeBuilder;

mod triangle;
use triangle::Triangle;

#[derive(Debug)]
pub struct Object {
    pub material: Material,
    transform: Matrix4D,
    kind: ObjectKind,
    id: u32,
}

#[derive(Debug)]
enum ObjectKind {
    Shape(Box<dyn Shape>),
    Group(Vec<Object>),
    Csg {
        left: Box<Object>,
        right: Box<Object>,
        operator: CsgOperator,
    },
}

#[derive(Debug, Copy, Clone)]
enum CsgOperator {
    Intersection,
    Subtract,
    Union,
}

impl CsgOperator {
    fn is_intersection(
        &self,
        intersected_left: bool,
        inside_left: bool,
        inside_right: bool,
    ) -> bool {
        match self {
            CsgOperator::Intersection => {
                (intersected_left && inside_right) || (!intersected_left && inside_left)
            }
            CsgOperator::Union => {
                (intersected_left && !inside_right) || (!intersected_left && !inside_left)
            }
            CsgOperator::Subtract => {
                (intersected_left && !inside_right) || (!intersected_left && inside_left)
            }
        }
    }
}

// if you need more than 4 billion objects, you've got bigger problems than integer overflow
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

impl Object {
    pub fn sphere() -> Self {
        Self::from_shape(Box::new(Sphere))
    }

    pub fn plane() -> Self {
        Self::from_shape(Box::new(Plane))
    }

    pub fn cube() -> Self {
        Self::from_shape(Box::new(Cube))
    }

    pub fn cylinder() -> CylinderBuilder {
        CylinderBuilder::new()
    }

    pub fn cone() -> ConeBuilder {
        ConeBuilder::new()
    }

    pub fn triangle(point1: Point3D, point2: Point3D, point3: Point3D) -> Self {
        Self::from_shape(Box::new(Triangle::new(point1, point2, point3)))
    }

    pub fn smooth_triangle(
        point1: Point3D,
        point2: Point3D,
        point3: Point3D,
        normal1: Vector3D,
        normal2: Vector3D,
        normal3: Vector3D,
    ) -> Self {
        Self::from_shape(Box::new(Triangle::smooth(
            point1, point2, point3, normal1, normal2, normal3,
        )))
    }

    pub fn group(children: Vec<Object>) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind: ObjectKind::Group(children),
            id: Self::next_id(),
        }
    }

    pub fn csg_union(left: Object, right: Object) -> Self {
        Self::csg(left, right, CsgOperator::Union)
    }

    pub fn csg_intersection(left: Object, right: Object) -> Self {
        Self::csg(left, right, CsgOperator::Intersection)
    }

    pub fn csg_difference(left: Object, mut right: Object) -> Self {
        right.material.casts_shadow = false;

        Self::csg(left, right, CsgOperator::Subtract)
    }

    fn csg(left: Object, right: Object, operator: CsgOperator) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind: ObjectKind::Csg {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            },
            id: Self::next_id(),
        }
    }

    fn from_shape(shape: Box<dyn Shape>) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind: ObjectKind::Shape(shape),
            id: Self::next_id(),
        }
    }

    fn next_id() -> u32 {
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }

    pub fn normal_at(&self, point: Point3D, uv: Option<(f64, f64)>) -> Vector3D {
        let inverted_transform = self
            .transform
            .inverse()
            .expect("transformation Matrix must be invertible");

        let (x, y, z, w) = &inverted_transform * point;

        debug_assert!(w == 1.0, "Point transformation did not return a point");
        let object_point = Point3D::new(x, y, z);
        let object_normal = match &self.kind {
            ObjectKind::Shape(shape) => shape.object_normal_at(object_point, uv),
            ObjectKind::Group(_) => unreachable!("should never need to calculate normals on Group object as rays should only intersect Shapes"),
            ObjectKind::Csg { .. } => unreachable!("Rays cannot intersect CSGs directly")
        };

        // deliberately ignoring `w` as a translation Matrix may affect `w` so it's no longer 0
        let (x, y, z, _) = inverted_transform.transpose() * object_normal;
        let world_normal = Vector3D::new(x, y, z);
        world_normal.normalised()
    }

    pub fn colour_at(
        &self,
        point: Point3D,
        light: &Light,
        eye_vector: Vector3D,
        surface_normal: Vector3D,
        in_shadow: bool,
    ) -> Colour {
        let material = &self.material;

        let object_point = {
            let inverse = self
                .transform
                .inverse()
                .expect("A transformation matrix must be invertible");
            let (x, y, z, _) = inverse * point;
            Point3D::new(x, y, z)
        };

        let colour = material.pattern.colour_at(object_point) * light.colour();
        let ambient = colour * material.ambient;

        if in_shadow {
            return ambient;
        }

        let light_vector = (light.position() - point).normalised();

        let light_dot_normal = light_vector.dot(surface_normal);
        // if dot product is <= 0, the light is behind the surface
        if light_dot_normal.is_sign_negative() {
            return ambient;
        }

        let diffuse = colour * material.diffuse * light_dot_normal;
        let reflected = (-light_vector).reflect_through(surface_normal);

        let reflect_dot_eye = reflected.dot(eye_vector);
        // if dot product is <= 0, the reflected light cannot reach the eye
        if reflect_dot_eye.is_sign_negative() {
            return ambient + diffuse;
        }

        let specular_factor = reflect_dot_eye.powf(material.shininess);
        let specular = light.colour() * material.specular * specular_factor;

        ambient + diffuse + specular
    }

    pub fn intersect(&self, with: &Ray) -> Intersections {
        debug_assert!(
            {
                let normalised = with.direction.normalised();
                let direction = &with.direction;

                direction.x() - normalised.x() <= f64::EPSILON
                    && direction.y() - normalised.y() <= f64::EPSILON
                    && direction.z() - normalised.z() <= f64::EPSILON
            },
            format!(
                "the Ray must be normalised before intersecting: {:?}, {:?}",
                with.direction,
                with.direction.normalised()
            )
        );

        match &self.kind {
            ObjectKind::Shape(shape) => {
                let ray_transform = self
                    .transform
                    .inverse()
                    .expect("A translation matrix should be invertible");

                let transformed = with.transformed(&ray_transform);
                let intersections = shape.object_intersect(&self, transformed);

                Intersections::of(intersections)
            }
            ObjectKind::Group(children) => children
                .iter()
                .map(|child| child.intersect(with))
                .fold(Intersections::empty(), |acc, next| acc.join(next)),
            ObjectKind::Csg {
                left,
                right,
                operator,
            } => {
                let left_intersections = left.intersect(&with);
                let right_intersections = right.intersect(&with);

                let intersections = left_intersections.join(right_intersections);

                let (mut in_left, mut in_right) = (false, false);
                let mut out = Intersections::empty();

                for intersection in intersections.into_iter() {
                    let left_hit = left.contains(intersection.with.id);

                    if operator.is_intersection(left_hit, in_left, in_right) {
                        out.push(intersection)
                    }

                    if left_hit {
                        in_left = !in_left;
                    } else {
                        in_right = !in_right;
                    }
                }

                out
            }
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    // TODO test that applying transforms to parents (i.e. CSG/Groups) affects children
    pub fn with_transform(mut self, transform: Matrix4D) -> Self {
        self.transform = transform;

        match &mut self.kind {
            ObjectKind::Group(children) => children
                .iter_mut()
                .for_each(|child| child.apply_transform(transform)),
            ObjectKind::Csg { left, right, .. } => {
                left.apply_transform(transform);
                right.apply_transform(transform);
            }
            _ => (),
        }

        self
    }

    /// allows a parent to push transforms applied to the group down to its children
    fn apply_transform(&mut self, transform: Matrix4D) {
        match &mut self.kind {
            ObjectKind::Group(children) => children
                .iter_mut()
                .for_each(|child| child.apply_transform(transform)),
            ObjectKind::Csg { left, right, .. } => {
                left.apply_transform(transform);
                right.apply_transform(transform);
            }
            _ => (),
        }

        self.transform = transform * self.transform;
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    fn contains(&self, id: u32) -> bool {
        match &self.kind {
            ObjectKind::Shape(_) => self.id == id,
            ObjectKind::Group(children) => children.iter().any(|child| child.contains(id)),
            ObjectKind::Csg { left, right, .. } => left.contains(id) || right.contains(id),
        }
    }
}

#[cfg(test)]
impl Object {
    pub fn children(&self) -> &Vec<Object> {
        if let ObjectKind::Group(children) = &self.kind {
            children
        } else {
            panic!("Object is not a group and has no children")
        }
    }

    pub fn shape(&self) -> &Box<dyn Shape> {
        if let ObjectKind::Shape(shape) = &self.kind {
            shape
        } else {
            panic!("Object is a group, not a shape")
        }
    }
}

pub trait Shape: Debug {
    fn object_normal_at(&self, point: Point3D, uv: Option<(f64, f64)>) -> Vector3D;
    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>>;
}

#[derive(Debug, PartialEq)]
/// A unit sphere, with the centre at the origin, and a radius of 1
struct Sphere;
impl Shape for Sphere {
    fn object_normal_at(&self, point: Point3D, _uv: Option<(f64, f64)>) -> Vector3D {
        point - Point3D::ORIGIN
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        let sphere_to_ray = with.origin - Point3D::ORIGIN;
        let a = with.direction.dot(with.direction);
        let b = 2.0 * with.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        if let Some((first, second)) = crate::util::quadratic(a, b, c) {
            vec![
                Intersection::new(first, parent),
                Intersection::new(second, parent),
            ]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
struct Plane;
impl Shape for Plane {
    fn object_normal_at(&self, _: Point3D, _uv: Option<(f64, f64)>) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
        if with.direction.y().abs() <= f32::EPSILON as f64 {
            return Vec::new();
        }

        let t = -with.origin.y() / with.direction.y();
        vec![Intersection::new(t, parent)]
    }
}

#[derive(Debug, PartialEq)]
struct Cube;
impl Shape for Cube {
    fn object_normal_at(&self, point: Point3D, _uv: Option<(f64, f64)>) -> Vector3D {
        if point.x().abs() >= point.y().abs() && point.x().abs() >= point.z().abs() {
            Vector3D::new(point.x(), 0.0, 0.0)
        } else if point.y().abs() >= point.x().abs() && point.y().abs() >= point.z().abs() {
            Vector3D::new(0.0, point.y(), 0.0)
        } else {
            Vector3D::new(0.0, 0.0, point.z())
        }
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Vec<Intersection<'parent>> {
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
            vec![]
        } else {
            vec![
                Intersection::new(t_min, parent),
                Intersection::new(t_max, parent),
            ]
        }
    }
}
