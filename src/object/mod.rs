use crate::{
    Colour, Intersection, Intersections, Material, Normal3D, Point3D, Ray, Transform, Vector,
    Vector3D,
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

mod bounds;
use crate::light::LightSample;
use crate::material::MaterialKind;
use crate::util::F64Ext;
use bounds::BoundingBox;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Object {
    pub material: Material,
    transform: Transform,
    kind: ObjectKind,
    bounds: BoundingBox,
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
        normal1: Normal3D,
        normal2: Normal3D,
        normal3: Normal3D,
    ) -> Self {
        Self::from_shape(Box::new(Triangle::smooth(
            point1, point2, point3, normal1, normal2, normal3,
        )))
    }

    pub fn group(children: Vec<Object>) -> Self {
        let initial_bounds = children
            .first()
            .map_or(BoundingBox::infinite(), |c| c.bounds);
        let bounds = children
            .iter()
            .skip(1)
            .map(|c| c.bounds)
            .fold(initial_bounds, |acc, next| acc.expand_to_fit(&next));

        Object {
            transform: Transform::identity(),
            material: Material::default(),
            bounds,
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
            transform: Transform::identity(),
            material: Material::default(),
            bounds: left.bounds.expand_to_fit(&right.bounds),
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
            transform: Transform::identity(),
            material: Material::default(),
            bounds: shape.object_bounds(),
            kind: ObjectKind::Shape(shape),
            id: Self::next_id(),
        }
    }

    fn next_id() -> u32 {
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }

    pub fn normal_at(&self, point: Point3D) -> Normal3D {
        let inverted_transform = self.transform.inverse();

        let (x, y, z, _) = inverted_transform * point;
        let object_point = Point3D::new(x, y, z);

        let object_normal = match &self.kind {
            ObjectKind::Shape(shape) => shape.object_normal_at(object_point),
            ObjectKind::Group(_) => unreachable!("should never need to calculate normals on Group object as rays should only intersect Shapes"),
            ObjectKind::Csg { .. } => unreachable!("Rays cannot intersect CSGs directly")
        };

        let (x, y, z, _) = inverted_transform.transpose() * object_normal;
        let world_normal = Vector3D::new(x, y, z);
        world_normal.normalised()
    }

    pub fn colour_at(
        &self,
        point: Point3D,
        direct_light: Colour,
        eye_vector: Normal3D,
        surface_normal: Normal3D,
        light_source: &LightSample,
    ) -> Colour {
        let material = &self.material;
        let material_colour = self.raw_colour_at(point);
        let ambient = material_colour * light_source.colour * material.ambient;

        // i.e. is in shadow
        if direct_light == Colour::BLACK {
            return ambient;
        }

        let light_vector = (light_source.position - point).normalised();
        let light_dot_normal = light_vector.dot(surface_normal);
        // if dot product is <= 0, the light is behind the surface
        if light_dot_normal.is_sign_negative() {
            return ambient;
        }

        let colour = material_colour * direct_light;
        let diffuse = colour * material.diffuse * light_dot_normal;

        let reflected = (-light_vector).reflect_through(surface_normal);
        let reflect_dot_eye = reflected.dot(eye_vector);
        // if dot product is <= 0, the reflected light cannot reach the eye
        if reflect_dot_eye.is_sign_negative() {
            return ambient + diffuse;
        }

        let specular_factor = reflect_dot_eye.powf(material.shininess);
        let specular = direct_light * material.specular * specular_factor;

        ambient + diffuse + specular
    }

    /// The colour of the object at the given point based on the object's material/pattern,
    /// without taking the lighting or eye location into account
    ///
    /// Intended for use by transparency/shadow calculations
    pub fn raw_colour_at(&self, point: Point3D) -> Colour {
        let object_point = {
            let inverse = self.transform.inverse();

            let (x, y, z, _) = inverse * point;
            Point3D::new(x, y, z)
        };

        match &self.material.kind {
            MaterialKind::Pattern(pattern) => pattern.colour_at(object_point),
            MaterialKind::Solid(colour) => *colour,
            MaterialKind::Uv(uv_pattern) => {
                let uv = match &self.kind {
                    ObjectKind::Shape(shape) => shape.uv_at(object_point),
                    ObjectKind::Group(_) => panic!("cannot UV map a group"),
                    ObjectKind::Csg { .. } => panic!("cannot UV map a CSG"),
                };

                uv_pattern.colour_at(uv)
            }
        }
    }

    pub fn intersect(&self, with: &Ray) -> Intersections {
        if !self.bounds.intersected_by(&with) {
            return Intersections::empty();
        }

        match &self.kind {
            ObjectKind::Shape(shape) => {
                let ray_transform = self.transform.inverse();

                let transformed = with.transformed(&ray_transform);
                shape.object_intersect(&self, transformed)
            }
            ObjectKind::Group(children) => children
                .iter()
                .map(|child| child.intersect(with))
                .fold(Intersections::empty(), Intersections::join),
            ObjectKind::Csg {
                left: first,
                right: second,
                operator,
            } => {
                // FIXME there must be a more idiomatic way to do this
                let first_intersections = first.intersect(&with);
                let second_intersections = second.intersect(&with);

                let intersections = first_intersections.join(second_intersections);

                let (mut in_first, mut in_second) = (false, false);
                let mut out = Intersections::empty();

                for intersection in intersections.into_iter() {
                    let hit_first = first.contains(intersection.with.id);

                    if operator.is_intersection(hit_first, in_first, in_second) {
                        out.push(intersection)
                    }

                    if hit_first {
                        in_first = !in_first;
                    } else {
                        in_second = !in_second;
                    }
                }

                out
            }
        }
    }

    /// Re-organises Group structures such that each sub-group contains no more than `threshold` members,
    /// as much as possible. If no sub-groups exist, they may be created, to respect the `threshold` value.
    ///
    /// (Sub) Groups are divided based on the bounding box - if a shape cannot neatly fit within a split
    /// bounding box, it will be kept in its current group, otherwise it will be moved into a sub-group
    /// dependent on which sub-bounding box contains it. Therefore, groups may be larger than `threshold`.
    pub fn optimised(mut self, threshold: usize) -> Self {
        self.kind = match self.kind {
            shape @ ObjectKind::Shape(_) => shape,
            ObjectKind::Group(children) => {
                let children = if children.len() >= threshold && children.len() > 1 {
                    // TODO test groups with planes (infinite shapes) can be optimised
                    let (left, right) = self.bounds.split();

                    let (left_fits, right_fits, mut neither_fits) = children.into_iter().fold(
                        (Vec::new(), Vec::new(), Vec::new()),
                        |(mut l, mut r, mut n), next| {
                            if left.fully_contains(&next.bounds) {
                                l.push(next)
                            } else if right.fully_contains(&next.bounds) {
                                r.push(next)
                            } else {
                                n.push(next)
                            }

                            (l, r, n)
                        },
                    );

                    if !left_fits.is_empty() {
                        neither_fits.push(Object::group(left_fits));
                    }
                    if !right_fits.is_empty() {
                        neither_fits.push(Object::group(right_fits));
                    }

                    neither_fits
                } else {
                    children
                };

                let children = children
                    .into_iter()
                    .map(|child| child.optimised(threshold))
                    .collect();

                ObjectKind::Group(children)
            }
            ObjectKind::Csg {
                left,
                right,
                operator,
            } => ObjectKind::Csg {
                left: Box::new(left.optimised(threshold)),
                right: Box::new(right.optimised(threshold)),
                operator,
            },
        };

        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.apply_material(material);
        self
    }

    fn apply_material(&mut self, material: Material) {
        match &mut self.kind {
            ObjectKind::Group(children) => children
                .iter_mut()
                .for_each(|child| child.apply_material(material.clone())),
            ObjectKind::Csg { left, right, .. } => {
                left.apply_material(material.clone());
                right.apply_material(material.clone());
            }
            _ => (),
        }

        self.material = material;
    }

    /// note that the provided `transform` is _combined_ with the existing transform
    /// such that the existing transform is applied first, then the provided one, as opposed to replacing it
    pub fn transformed(mut self, transform: Transform) -> Self {
        self.apply_transform(transform);

        self
    }

    /// `with_transform` is designed to be used as a fluent API, and therefore takes ownership (then returns it),
    /// but requires re-assignment, whereas this function makes it clear that the `Object` is mutated in-place
    fn apply_transform(&mut self, transform: Transform) {
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
        self.bounds = self.bounds.transformed(transform);
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
            panic!("Object is not a shape")
        }
    }

    pub fn csg_children(&self) -> (&Box<Object>, &Box<Object>) {
        if let ObjectKind::Csg { left, right, .. } = &self.kind {
            (left, right)
        } else {
            panic!("Object is not a CSG")
        }
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }
}

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

#[derive(Debug, PartialEq)]
/// A unit sphere, with the centre at the origin, and a radius of 1
struct Sphere;
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

        if let Some((first, second)) = crate::util::quadratic(a, b, c) {
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

#[derive(Debug, PartialEq)]
// an infinite XZ plane
struct Plane;
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

#[derive(Debug, PartialEq)]
// a 2x2x2 cube, centred at the world Origin (i.e. from (-1, -1, -1) to (1, 1, 1))
struct Cube;
impl Shape for Cube {
    fn object_bounds(&self) -> BoundingBox {
        BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0))
    }

    fn object_normal_at(&self, point: Point3D) -> Normal3D {
        if point.x().abs() >= point.y().abs() && point.x().abs() >= point.z().abs() {
            Vector3D::new(point.x(), 0.0, 0.0)
        } else if point.y().abs() >= point.x().abs() && point.y().abs() >= point.z().abs() {
            Vector3D::new(0.0, point.y(), 0.0)
        } else {
            Vector3D::new(0.0, 0.0, point.z())
        }
        .normalised()
    }

    fn object_intersect<'parent>(
        &self,
        parent: &'parent Object,
        with: Ray,
    ) -> Intersections<'parent> {
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
            Intersections::empty()
        } else {
            Intersections::pair(
                Intersection::new(t_min, parent),
                Intersection::new(t_max, parent),
            )
        }
    }

    /// ranges from u <- 0..3 and v <- 0..4 such that:
    ///  - u <- 1..2; v <- 0..1 maps to the top face
    ///  - u <- 1..2; v <- 1..2 maps to the right face
    ///  - u <- 0..1; v <- 2..3 maps to the front face
    ///  - u <- 1..2; v <- 2..3 maps to the bottom face
    ///  - u <- 2..3; v <- 2..3 maps to the back face
    ///  - u <- 1..2; v <- 3..4 maps to the left face
    fn uv_at(&self, point: Point3D) -> (f64, f64) {
        let largest = point.x().abs().max(point.y().abs().max(point.z().abs()));

        if largest == point.x() {
            // right face
            let u = (1.0 - point.z()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 1.0)
        } else if largest == -point.x() {
            // left face
            let u = (1.0 + point.z()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 3.0)
        } else if largest == point.y() {
            // top face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 - point.z()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v)
        } else if largest == -point.y() {
            // bottom face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.z()).rem_euclid(2.0) / 2.0;

            (u + 1.0, v + 2.0)
        } else if largest == point.z() {
            // front face
            let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u, v + 2.0)
        } else {
            // back face
            let u = (1.0 - point.x()).rem_euclid(2.0) / 2.0;
            let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

            (u + 2.0, v + 2.0)
        }
    }
}
