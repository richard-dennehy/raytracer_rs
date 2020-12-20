use crate::{Colour, Material, Matrix4D, Point3D, PointLight, Vector3D};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4D,
    pub material: Material,
}

impl Sphere {
    pub fn unit() -> Self {
        Sphere {
            transform: Matrix4D::identity(),
            material: Material::default(),
        }
    }

    pub fn transform(&mut self, transform: Matrix4D) {
        self.transform = transform
    }

    pub fn normal_at(&self, point: Point3D) -> Vector3D {
        let inverted_transform = self
            .transform
            .inverse()
            .expect("transformation Matrix must be invertible");

        let (x, y, z, w) = &inverted_transform * point;

        debug_assert!(w == 1.0, "Point transformation did not return a point");
        let object_point = Point3D::new(x, y, z);
        let object_normal = object_point - Point3D::new(0.0, 0.0, 0.0); // sphere origin

        // deliberately ignoring `w` as a translation Matrix may affect `w` so it's no longer 0
        let (x, y, z, _) = inverted_transform.transpose() * object_normal;
        let world_normal = Vector3D::new(x, y, z);
        world_normal.normalised()
    }

    pub fn colour_at(&self, point: Point3D, light: &PointLight, eye_vector: Vector3D) -> Colour {
        let surface_normal = self.normal_at(point);
        let material = &self.material;

        let colour = material.colour * light.intensity;
        let light_vector = (light.position - point).normalised();

        let ambient = colour * material.ambient;

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
}
