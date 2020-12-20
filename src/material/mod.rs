use crate::{Colour, Point3D, PointLight, Vector3D};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn with_light(
        &self,
        light: &PointLight,
        point: Point3D,
        eye_vector: Vector3D,
        surface_normal: Vector3D,
    ) -> Colour {
        let colour = self.colour * light.intensity;
        let light_vector = (light.position - point).normalised();

        let ambient = colour * self.ambient;

        let light_dot_normal = light_vector.dot(&surface_normal);
        // if dot product is <= 0, the light is behind the surface
        if light_dot_normal.is_sign_negative() {
            return ambient;
        }

        let diffuse = colour * self.diffuse * light_dot_normal;
        let reflected = (-light_vector).reflect_through(surface_normal);

        let reflect_dot_eye = reflected.dot(&eye_vector);
        // if dot product is <= 0, the reflected light cannot reach the eye
        if reflect_dot_eye.is_sign_negative() {
            return ambient + diffuse;
        }

        let specular_factor = reflect_dot_eye.powf(self.shininess);
        let specular = light.intensity * self.specular * specular_factor;

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            colour: Colour::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
