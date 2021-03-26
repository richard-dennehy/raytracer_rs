use crate::ray::HitData;
use crate::{
    Colour, Intersections, Light, Material, Object, Pattern, Point3D, Ray, Transform, Vector,
};

#[cfg(test)]
mod tests;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub settings: WorldSettings,
}

pub struct WorldSettings {
    /// Max number of rays to cast from reflections/refractions
    /// Higher values produce more accurate results, but increase rendering time
    pub recursion_depth: u8,
    /// Default colour returned when a ray doesn't intersect any objects
    pub sky_colour: Colour,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            recursion_depth: 5,
            sky_colour: Colour::BLACK,
        }
    }
}

impl World {
    pub fn empty() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
            settings: Default::default(),
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
            settings: Default::default(),
        }
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        fn inner(this: &World, ray: Ray, limit: u8) -> Colour {
            if limit == 0 {
                return Colour::BLACK;
            }

            let intersections = this.intersect(&ray);
            if let Some(hit) = intersections.hit() {
                let hit_data = HitData::from(&ray, hit, intersections);
                let surface = this.shade_hit(&hit_data);

                let reflected = if hit_data.object.material.reflective == 0.0 {
                    Colour::BLACK
                } else {
                    let reflection_vector =
                        ray.direction.normalised().reflect_through(hit_data.normal);
                    let reflection = Ray::new(hit_data.over_point, reflection_vector);
                    inner(this, reflection, limit - 1) * hit_data.object.material.reflective
                };

                if hit_data.object.material.transparency == 0.0 {
                    surface + reflected
                } else {
                    // check for total internal reflection
                    let reflection_data = hit_data.reflection();

                    let refracted = if reflection_data.is_total() {
                        Colour::BLACK
                    } else {
                        let refracted_direction =
                            reflection_data.refraction_vector(hit_data.normal, hit_data.eye);

                        let refracted_ray =
                            Ray::new(hit_data.under_point, refracted_direction.normalised());

                        inner(this, refracted_ray, limit - 1)
                            * hit_data.object.material.transparency
                    };

                    if hit_data.object.material.reflective > 0.0 {
                        let reflectance = reflection_data
                            .reflectance(hit_data.entered_refractive, hit_data.exited_refractive);

                        surface + (reflected * reflectance) + (refracted * (1.0 - reflectance))
                    } else {
                        surface + reflected + refracted
                    }
                }
            } else {
                this.settings.sky_colour
            }
        }

        inner(self, ray, self.settings.recursion_depth)
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
            .map(|light| {
                let direct_light = self.direct_light(hit_data.over_point, light);

                hit_data.colour(direct_light, light)
            })
            .sum()
    }

    fn direct_light(&self, point: Point3D, light: &Light) -> Colour {
        let light_vector = light.position() - point;
        let light_distance = light_vector.magnitude();
        let light_vector = light_vector.normalised();

        let ray = Ray::new(point, light_vector);

        self.intersect(&ray)
            .into_iter()
            .filter(|i| i.t >= 0.0 && i.t < light_distance)
            .fold(light.colour(), |light, hit| {
                if hit.with.material.casts_shadow {
                    // TODO should be affected by transparent material colour
                    // TODO figure out what colour the transparent material is at intersection point
                    light * hit.with.material.transparency
                } else {
                    light
                }
            })
    }
}
