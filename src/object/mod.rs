use crate::{
    Colour, Intersection, Intersections, Material, Matrix4D, Point3D, PointLight, Ray, Vector3D,
};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub transform: Matrix4D,
    pub material: Material,
    kind: Shape,
}

impl Object {
    pub fn sphere() -> Self {
        Self::new(Shape::Sphere)
    }

    pub fn plane() -> Self {
        Self::new(Shape::Plane)
    }

    pub fn cube() -> Self {
        Self::new(Shape::Cube)
    }

    fn new(kind: Shape) -> Self {
        Object {
            transform: Matrix4D::identity(),
            material: Material::default(),
            kind,
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
        let object_normal = self.kind.object_normal_at(object_point);

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
        let ray_transform = self
            .transform
            .inverse()
            .expect("A translation matrix should be invertible");

        let transformed = with.transformed(&ray_transform);

        let ts = self.kind.object_intersect(transformed);
        let intersections = ts
            .into_iter()
            .map(|t| Intersection::new(t, &self))
            .collect();

        Intersections::of(intersections)
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_transform(mut self, transform: Matrix4D) -> Self {
        self.transform = transform;
        self
    }
}

#[derive(Debug, PartialEq)]
enum Shape {
    Sphere,
    Plane,
    Cube,
}

impl Shape {
    pub fn object_normal_at(&self, point: Point3D) -> Vector3D {
        match *self {
            Shape::Sphere => point - Point3D::new(0.0, 0.0, 0.0),
            Shape::Plane => Vector3D::new(0.0, 1.0, 0.0),
            Shape::Cube => Self::cube_normal(point),
        }
    }

    fn cube_normal(point: Point3D) -> Vector3D {
        if point.x().abs() >= point.y().abs() && point.x().abs() >= point.z().abs() {
            Vector3D::new(point.x(), 0.0, 0.0)
        } else if point.y().abs() >= point.x().abs() && point.y().abs() >= point.z().abs() {
            Vector3D::new(0.0, point.y(), 0.0)
        } else {
            Vector3D::new(0.0, 0.0, point.z())
        }
    }

    pub fn object_intersect(&self, with: Ray) -> Vec<f64> {
        match *self {
            Shape::Sphere => Shape::sphere_intersect(with),
            Shape::Plane => Shape::plane_intersect(with),
            Shape::Cube => Shape::cube_intersect(with),
        }
    }

    fn sphere_intersect(with: Ray) -> Vec<f64> {
        let sphere_to_ray = with.origin - Point3D::new(0.0, 0.0, 0.0);
        let a = with.direction.dot(&with.direction);
        let b = 2.0 * with.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![t1, t2]
    }

    fn plane_intersect(with: Ray) -> Vec<f64> {
        if with.direction.y().abs() <= f32::EPSILON as f64 {
            return Vec::new();
        }

        vec![-with.origin.y() / with.direction.y()]
    }

    fn cube_intersect(with: Ray) -> Vec<f64> {
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
