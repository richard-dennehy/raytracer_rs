use crate::{
    Colour, Intersection, Intersections, Material, Matrix4D, Point3D, PointLight, Ray, Vector3D,
};
use std::fmt::Debug;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(test)]
mod tests;

mod cylinder;
pub use cylinder::CylinderBuilder;

mod cone;
pub use cone::ConeBuilder;

#[derive(Debug)]
pub struct Object {
    // FIXME material makes no sense on groups - move into `Shape`
    pub material: Material,
    transform: Matrix4D,
    kind: ObjectKind,
    // FIXME id arguably makes no sense on groups - move into `Shape`
    id: u32,
}

#[derive(Debug)]
enum ObjectKind {
    Shape(Box<dyn Shape>),
    Group(Vec<Object>),
}

// if you need more than 4 billion objects, you've got bigger problems than integer overflow
static NEXT_ID: AtomicU32 = AtomicU32::new(0);

impl Object {
    pub fn sphere() -> Self {
        Self::shape(Box::new(Sphere))
    }

    pub fn plane() -> Self {
        Self::shape(Box::new(Plane))
    }

    pub fn cube() -> Self {
        Self::shape(Box::new(Cube))
    }

    pub fn cylinder() -> CylinderBuilder {
        CylinderBuilder::new()
    }

    pub fn cone() -> ConeBuilder {
        ConeBuilder::new()
    }

    pub fn group(children: Vec<Object>) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind: ObjectKind::Group(children),
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    fn shape(shape: Box<dyn Shape>) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind: ObjectKind::Shape(shape),
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    pub fn normal_at(&self, point: Point3D) -> Vector3D {
        let inverted_transform = self
            .transform
            .inverse()
            .expect("transformation Matrix must be invertible");

        let (x, y, z, w) = &inverted_transform * point;

        debug_assert!(w == 1.0, "Point transformation did not return a point");
        let object_point = Point3D::new(x, y, z);
        let object_normal = match &self.kind {
            ObjectKind::Shape(shape) => shape.object_normal_at(object_point),
            ObjectKind::Group(_) => panic!("should never need to calculate normals on Group object as rays should only intersect Shapes")
        };

        // deliberately ignoring `w` as a translation Matrix may affect `w` so it's no longer 0
        let (x, y, z, _) = inverted_transform.transpose() * object_normal;
        let world_normal = Vector3D::new(x, y, z);
        world_normal.normalised()
    }

    pub fn colour_at(
        &self,
        point: Point3D,
        light: &PointLight,
        eye_vector: Vector3D,
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

        let colour = material.pattern.colour_at(object_point) * light.intensity;
        let ambient = colour * material.ambient;

        if in_shadow {
            return ambient;
        }

        let light_vector = (light.position - point).normalised();
        // FIXME calculating inverse multiple times
        let surface_normal = self.normal_at(point);

        let light_dot_normal = light_vector.dot(&surface_normal);
        // if dot product is <= 0, the light is behind the surface
        if light_dot_normal.is_sign_negative() {
            return ambient;
        }

        let diffuse = colour * material.diffuse * light_dot_normal;
        let reflected = (-light_vector).reflect_through(surface_normal);

        let reflect_dot_eye = reflected.dot(&eye_vector);
        // if dot product is <= 0, the reflected light cannot reach the eye
        if reflect_dot_eye.is_sign_negative() {
            return ambient + diffuse;
        }

        let specular_factor = reflect_dot_eye.powf(material.shininess);
        let specular = light.intensity * material.specular * specular_factor;

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

                let ts = shape.object_intersect(transformed);
                let intersections = ts
                    .into_iter()
                    .map(|t| Intersection::new(t, &self))
                    .collect();

                Intersections::of(intersections)
            }
            ObjectKind::Group(children) => children
                .iter()
                .map(|child| child.intersect(with))
                .fold(Intersections::empty(), |acc, next| acc.join(next)),
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_transform(mut self, transform: Matrix4D) -> Self {
        self.transform = transform;

        if let ObjectKind::Group(children) = &mut self.kind {
            children
                .iter_mut()
                .for_each(|child| child.apply_transform(transform))
        };

        self
    }

    /// allows a parent `Group` to push transforms applied to the group down to its children
    fn apply_transform(&mut self, transform: Matrix4D) {
        if let ObjectKind::Group(children) = &mut self.kind {
            children
                .iter_mut()
                .for_each(|child| child.apply_transform(transform))
        }

        self.transform = transform * self.transform;
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

pub trait Shape: Debug {
    fn object_normal_at(&self, point: Point3D) -> Vector3D;
    fn object_intersect(&self, with: Ray) -> Vec<f64>;
}

#[derive(Debug)]
struct Sphere;
impl Shape for Sphere {
    fn object_normal_at(&self, point: Point3D) -> Vector3D {
        point - Point3D::ORIGIN
    }

    fn object_intersect(&self, with: Ray) -> Vec<f64> {
        let sphere_to_ray = with.origin - Point3D::ORIGIN;
        let a = with.direction.dot(&with.direction);
        let b = 2.0 * with.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        if let Some((first, second)) = crate::util::quadratic(a, b, c) {
            vec![first, second]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
struct Plane;
impl Shape for Plane {
    fn object_normal_at(&self, _: Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }

    fn object_intersect(&self, with: Ray) -> Vec<f64> {
        if with.direction.y().abs() <= f32::EPSILON as f64 {
            return Vec::new();
        }

        vec![-with.origin.y() / with.direction.y()]
    }
}

#[derive(Debug)]
struct Cube;
impl Shape for Cube {
    fn object_normal_at(&self, point: Point3D) -> Vector3D {
        if point.x().abs() >= point.y().abs() && point.x().abs() >= point.z().abs() {
            Vector3D::new(point.x(), 0.0, 0.0)
        } else if point.y().abs() >= point.x().abs() && point.y().abs() >= point.z().abs() {
            Vector3D::new(0.0, point.y(), 0.0)
        } else {
            Vector3D::new(0.0, 0.0, point.z())
        }
    }

    fn object_intersect(&self, with: Ray) -> Vec<f64> {
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
            vec![t_min, t_max]
        }
    }
}
