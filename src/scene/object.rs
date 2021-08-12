use super::*;
use crate::core::{Colour, Normal3D, Point3D, Ray, Transform, Vector3D, VectorMaths};
use crate::scene::{Material, MaterialKind};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct Object {
    pub material: Material,
    transform: Transform,
    kind: ObjectKind,
    pub(in crate::scene) bounds: BoundingBox,
    pub(in crate::scene) id: u32,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CsgOperator {
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

    pub(in crate::scene) fn from_shape(shape: Box<dyn Shape>) -> Self {
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
                    ObjectKind::Shape(shape) => {
                        // this isn't the best place (or at least, it's inconsistent),
                        // but it's not obvious how to apply the inverse once the Point has been converted to a UV
                        let inverse = uv_pattern.transform.inverse();

                        let (x, y, z, _) = inverse * object_point;
                        shape.uv_at(Point3D::new(x, y, z))
                    }
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
                let first_intersections = first.intersect(&with);
                let second_intersections = second.intersect(&with);

                let intersections = first_intersections.join(second_intersections);

                let (filtered, ..) = intersections.into_iter().fold(
                    (Intersections::empty(), false, false),
                    |(mut out, in_first, in_second), intersection| {
                        let hit_first = first.contains(intersection.with.id);

                        if operator.is_intersection(hit_first, in_first, in_second) {
                            out.push(intersection)
                        };

                        if hit_first {
                            (out, !in_first, in_second)
                        } else {
                            (out, in_first, !in_second)
                        }
                    },
                );

                filtered
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
