use crate::ray::HitData;
use crate::{Colour, Intersections, Material, Matrix4D, Object, Pattern, Point3D, PointLight, Ray};

#[cfg(test)]
mod tests;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub const fn empty() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn default() -> Self {
        World {
            objects: vec![
                Object::sphere().with_material(Material::new(
                    Pattern::solid(Colour::new(0.8, 1.0, 0.6)),
                    0.1,
                    0.7,
                    0.2,
                    200.0,
                )),
                Object::sphere().with_transform(Matrix4D::uniform_scaling(0.5)),
            ],
            lights: vec![PointLight::new(
                Colour::WHITE,
                Point3D::new(-10.0, 10.0, -10.0),
            )],
        }
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        let intersections = self.intersect(&ray);
        if let Some(hit) = intersections.hit() {
            let hit_data = ray.hit_data(hit);
            self.shade_hit(&hit_data)
        } else {
            Colour::BLACK
        }
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        self.objects
            .iter()
            .map(|obj| obj.intersect(&ray))
            .fold(Intersections::empty(), Intersections::join)
    }

    fn shade_hit(&self, hit_data: &HitData) -> Colour {
        self.lights
            .iter()
            .map(|light| hit_data.colour(light, self.is_in_shadow(hit_data.shadow_point, light)))
            .sum()
    }

    fn is_in_shadow(&self, point: Point3D, light: &PointLight) -> bool {
        let light_vector = light.position - point;
        let light_distance = light_vector.magnitude();
        let light_vector = light_vector.normalised();

        let ray = Ray::new(point, light_vector);
        if let Some(hit) = self.intersect(&ray).hit() {
            hit.t < light_distance
        } else {
            false
        }
    }
}
