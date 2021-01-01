use crate::ray::HitData;
use crate::{Colour, Intersections, Material, Matrix4D, Point3D, PointLight, Ray, Sphere};

#[cfg(test)]
mod tests;

pub struct World {
    pub objects: Vec<Sphere>,
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
                Sphere::with_material(Material::new(
                    Colour::new(0.8, 1.0, 0.6),
                    0.1,
                    0.7,
                    0.2,
                    200.0,
                )),
                Sphere::with_transform(Matrix4D::uniform_scaling(0.5)),
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
            .filter_map(|obj| ray.intersect(obj))
            .fold(Intersections::empty(), |acc, (first, second)| {
                acc.push(first, second)
            })
    }

    fn shade_hit(&self, hit_data: &HitData) -> Colour {
        self.lights.iter().map(|light| hit_data.colour(light)).sum()
    }
}
