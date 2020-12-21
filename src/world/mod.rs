use crate::{Colour, Intersections, Material, Matrix4D, Point3D, PointLight, Ray, Sphere};

#[cfg(test)]
mod tests;

pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
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

    pub fn intersect(&self, ray: Ray) -> Intersections {
        self.objects
            .iter()
            .filter_map(|obj| ray.intersect(obj))
            .fold(Intersections::empty(), |acc, (first, second)| {
                acc.push(first, second)
            })
    }
}
