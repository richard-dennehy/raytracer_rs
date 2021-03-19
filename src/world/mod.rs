use crate::ray::HitData;
use crate::{
    Colour, Intersections, Light, Material, Object, Pattern, Point3D, Ray, Transform, Vector,
};

#[cfg(test)]
mod tests;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
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
                Object::sphere().with_material(Material {
                    pattern: Pattern::solid(Colour::new(0.8, 1.0, 0.6)),
                    ambient: 0.1,
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Default::default()
                }),
                Object::sphere().transformed(Transform::identity().scale_all(0.5)),
            ],
            lights: vec![Light::point(
                Colour::WHITE,
                Point3D::new(-10.0, 10.0, -10.0),
            )],
        }
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        fn inner(this: &World, ray: Ray, limit: usize) -> Colour {
            if limit == 0 {
                return Colour::BLACK;
            }

            let intersections = this.intersect(&ray);
            if let Some(hit) = intersections.hit() {
                let hit_data = HitData::from(&ray, hit, intersections);
                let surface = this.shade_hit(&hit_data);

                let reflective = if hit_data.object.material.reflective == 0.0 {
                    Colour::BLACK
                } else {
                    let reflection = Ray::new(hit_data.over_point, hit_data.reflection);
                    inner(this, reflection, limit - 1) * hit_data.object.material.reflective
                };

                let refractive = if hit_data.object.material.transparency == 0.0 {
                    Colour::BLACK
                } else {
                    // check for total internal reflection
                    let ratio = hit_data.entered_refractive / hit_data.exited_refractive;
                    let cos_i = hit_data.eye.dot(hit_data.normal);
                    let sin2_t = ratio.powi(2) * (1.0 - cos_i.powi(2));

                    if sin2_t > 1.0 {
                        Colour::BLACK
                    } else {
                        let cos_t = (1.0 - sin2_t).sqrt();
                        let refracted_direction =
                            hit_data.normal * (ratio * cos_i - cos_t) - (hit_data.eye * ratio);

                        let refracted_ray =
                            Ray::new(hit_data.under_point, refracted_direction.normalised());

                        inner(this, refracted_ray, limit - 1)
                            * hit_data.object.material.transparency
                    }
                };

                if hit_data.object.material.reflective > 0.0
                    && hit_data.object.material.transparency > 0.0
                {
                    let reflectance = hit_data.reflectance();

                    surface + (reflective * reflectance) + (refractive * (1.0 - reflectance))
                } else {
                    surface + reflective + refractive
                }
            } else {
                Colour::BLACK
            }
        }

        inner(self, ray, 5)
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
            .map(|light| hit_data.colour(light, self.is_in_shadow(hit_data.over_point, light)))
            .sum()
    }

    fn is_in_shadow(&self, point: Point3D, light: &Light) -> bool {
        let light_vector = light.position() - point;
        let light_distance = light_vector.magnitude();
        let light_vector = light_vector.normalised();

        let ray = Ray::new(point, light_vector);
        if let Some(hit) = self.intersect(&ray).shadow_hit() {
            hit.t < light_distance
        } else {
            false
        }
    }
}
