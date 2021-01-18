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

    pub fn infinite_cylinder() -> Self {
        Self::new(Shape::Cylinder {
            max_y: f64::INFINITY,
            min_y: -f64::INFINITY,
            capped: false,
        })
    }

    pub fn hollow_cylinder(min_y: f64, max_y: f64) -> Self {
        Self::new(Shape::Cylinder {
            max_y,
            min_y,
            capped: false,
        })
    }

    pub fn capped_cylinder(min_y: f64, max_y: f64) -> Self {
        Self::new(Shape::Cylinder {
            max_y,
            min_y,
            capped: true,
        })
    }

    pub fn double_napped_cone() -> Self {
        Self::new(Shape::Cone {
            max_y: f64::INFINITY,
            min_y: -f64::INFINITY,
            capped: false,
        })
    }

    pub fn truncated_cone(min_y: f64, max_y: f64) -> Self {
        Self::new(Shape::Cone {
            min_y,
            max_y,
            capped: false,
        })
    }

    pub fn capped_cone(min_y: f64, max_y: f64) -> Self {
        Self::new(Shape::Cone {
            min_y,
            max_y,
            capped: true,
        })
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
    Cylinder {
        max_y: f64,
        min_y: f64,
        capped: bool,
    },
    Cone {
        max_y: f64,
        min_y: f64,
        capped: bool,
    },
}

impl Shape {
    pub fn object_normal_at(&self, point: Point3D) -> Vector3D {
        match *self {
            Shape::Sphere => point - Point3D::new(0.0, 0.0, 0.0),
            Shape::Plane => Vector3D::new(0.0, 1.0, 0.0),
            Shape::Cube => Self::cube_normal(point),
            Shape::Cylinder { min_y, max_y, .. } => Self::cylinder_normal(point, min_y, max_y),
            Shape::Cone { min_y, max_y, .. } => Self::cone_normal(point, min_y, max_y),
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

    fn cylinder_normal(point: Point3D, min_y: f64, max_y: f64) -> Vector3D {
        let distance = point.x().powi(2) + point.z().powi(2);

        if distance < 1.0 && point.y() >= max_y - f64::EPSILON {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if distance < 1.0 && point.y() <= min_y + f64::EPSILON {
            Vector3D::new(0.0, -1.0, 0.0)
        } else {
            Vector3D::new(point.x(), 0.0, point.z())
        }
    }

    fn cone_normal(point: Point3D, min_y: f64, max_y: f64) -> Vector3D {
        todo!()
    }

    pub fn object_intersect(&self, with: Ray) -> Vec<f64> {
        match *self {
            Shape::Sphere => Shape::sphere_intersect(with),
            Shape::Plane => Shape::plane_intersect(with),
            Shape::Cube => Shape::cube_intersect(with),
            Shape::Cylinder {
                min_y,
                max_y,
                capped,
            } => Self::cylinder_intersect(with, min_y, max_y, capped),
            Shape::Cone {
                min_y,
                max_y,
                capped,
            } => Self::cone_intersect(with, min_y, max_y, capped),
        }
    }

    fn sphere_intersect(with: Ray) -> Vec<f64> {
        let sphere_to_ray = with.origin - Point3D::new(0.0, 0.0, 0.0);
        let a = with.direction.dot(&with.direction);
        let b = 2.0 * with.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

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

    fn cylinder_intersect(with: Ray, min_y: f64, max_y: f64, capped: bool) -> Vec<f64> {
        let intersects_cap = |t: f64| {
            let x = with.origin.x() + t * with.direction.x();
            let z = with.origin.z() + t * with.direction.z();

            (x.powi(2) + z.powi(2)) <= 1.0
        };

        let mut cap_intersections = if capped {
            let mut ts = Vec::with_capacity(2);
            // check bottom cap
            let t = (min_y - with.origin.y()) / with.direction.y();

            if intersects_cap(t) {
                ts.push(t);
            }

            // check top cap
            let t = (max_y - with.origin.y()) / with.direction.y();

            if intersects_cap(t) {
                ts.push(t);
            }

            ts
        } else {
            vec![]
        };

        let a = with.direction.x().powi(2) + with.direction.z().powi(2);

        if a.abs() <= f64::EPSILON {
            return cap_intersections;
        };

        let b =
            2.0 * with.origin.x() * with.direction.x() + 2.0 * with.origin.z() * with.direction.z();
        let c = with.origin.x().powi(2) + with.origin.z().powi(2) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        };

        let first = (-b - discriminant.sqrt()) / (2.0 * a);
        let second = (-b + discriminant.sqrt()) / (2.0 * a);

        let y_first = with.origin.y() + with.direction.y() * first;
        let y_second = with.origin.y() + with.direction.y() * second;

        let mut ts = Vec::with_capacity(2);
        if y_first > min_y && y_first < max_y {
            ts.push(first);
        }

        if y_second > min_y && y_second < max_y {
            ts.push(second);
        }

        ts.append(&mut cap_intersections);

        ts
    }

    fn cone_intersect(with: Ray, min_y: f64, max_y: f64, capped: bool) -> Vec<f64> {
        let a =
            with.direction.x().powi(2) - with.direction.y().powi(2) + with.direction.z().powi(2);
        let b = 2.0 * with.origin.x() * with.direction.x()
            - 2.0 * with.origin.y() * with.direction.y()
            + 2.0 * with.origin.z() * with.direction.z();

        let c = with.origin.x().powi(2) - with.origin.y().powi(2) + with.origin.z().powi(2);

        if a.abs() <= f64::EPSILON && b.abs() <= f64::EPSILON {
            return vec![];
        };

        if a.abs() <= f64::EPSILON {
            return vec![-c / (2.0 * b)];
        };

        if let Some((first, second)) = crate::util::quadratic(a, b, c) {
            vec![first, second]
        } else {
            vec![]
        }
    }
}
