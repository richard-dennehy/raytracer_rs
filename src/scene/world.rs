use crate::core::*;
use crate::scene::intersection::{HitData, Intersections};
use crate::scene::Material;
use crate::scene::MaterialKind;
use crate::scene::Object;
use crate::scene::{Light, LightSample};

pub struct World {
    pub(super) objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub settings: WorldSettings,
}

pub struct WorldSettings {
    /// Max number of rays to cast from reflections/refractions
    /// Higher values produce more accurate results, but increase rendering time
    pub recursion_depth: u8,
    /// Default colour returned when a ray doesn't intersect any objects
    pub sky_colour: Colour,
    /// how strongly the colour of a transparent material should affect the light passing through - works best with low values
    pub transparent_colour_tint: f64,
    /// the soft limit of group sizes - lower values will create more, smaller, bounding boxes, which speeds up rendering of
    /// more complex scenes, but potentially increases rendering time of very simple scenes
    pub group_size_threshold: u8,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            recursion_depth: 5,
            sky_colour: Colour::BLACK,
            transparent_colour_tint: 0.1,
            group_size_threshold: 4,
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
                    kind: MaterialKind::Solid(Colour::new(0.8, 1.0, 0.6)),
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

    pub fn add(&mut self, object: Object) {
        self.objects
            .push(object.optimised(self.settings.group_size_threshold as _));
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        fn inner(this: &World, ray: Ray, last_hit: Option<u32>, limit: u8) -> Colour {
            if limit == 0 {
                return Colour::BLACK;
            }

            let intersections = this.intersect(&ray);
            if let Some(hit) = intersections.hit(last_hit) {
                let hit_data = HitData::from(&ray, hit, intersections);
                let surface = this.shade_hit(&hit_data);

                let reflected = if hit_data.object.material.reflective == 0.0 {
                    Colour::BLACK
                } else {
                    let reflection_vector =
                        ray.direction.normalised().reflect_through(hit_data.normal);
                    let reflection = Ray::new(hit_data.point, reflection_vector);
                    inner(this, reflection, Some(hit_data.object.id()), limit - 1)
                        * hit_data.object.material.reflective
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
                            Ray::new(hit_data.point, refracted_direction.normalised());

                        inner(this, refracted_ray, Some(hit_data.object.id()), limit - 1)
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

        inner(self, ray, None, self.settings.recursion_depth)
    }

    pub(super) fn intersect(&self, ray: &Ray) -> Intersections {
        self.objects
            .iter()
            .map(|obj| obj.intersect(&ray))
            .fold(Intersections::empty(), Intersections::join)
    }

    pub(super) fn shade_hit(&self, hit_data: &HitData) -> Colour {
        self.lights
            .iter()
            .map(|light| {
                let (samples, n_samples) = light.samples();

                let sum = samples
                    .map(|point| {
                        let sample = LightSample::new(*point, light.colour());
                        let direct_light =
                            self.direct_light(hit_data.point, &sample, hit_data.object.id());

                        hit_data.colour(direct_light, &sample)
                    })
                    .sum::<Colour>();

                sum / (n_samples as f64)
            })
            .sum()
    }

    fn direct_light(&self, point: Point3D, light: &LightSample, target_id: u32) -> Colour {
        let light_vector = light.position - point;
        let light_distance = light_vector.magnitude();

        // if light source is exactly at the intersection point, use full intensity
        if light_distance.is_roughly_zero() {
            return light.colour;
        }

        let light_vector = light_vector.normalised();

        let ray = Ray::new(point, light_vector);

        self.intersect(&ray)
            .into_iter()
            .filter(|i| i.with.id() != target_id || i.t.is_not_roughly_zero())
            .filter(|i| i.t >= 0.0 && i.t < light_distance)
            .fold(light.colour, |light, hit| {
                if light == Colour::BLACK {
                    return Colour::BLACK;
                }

                if hit.with.material.casts_shadow {
                    // opaque object prevents light from reaching point
                    if hit.with.material.transparency == 0.0 {
                        return Colour::BLACK;
                    }

                    let hit_colour = hit.with.raw_colour_at(ray.position(hit.t));
                    // plain glass, etc, don't have a colour, and shouldn't change the colour of light passing though
                    if hit_colour == Colour::BLACK {
                        return light * hit.with.material.transparency;
                    }
                    // This colour mixing is very crude, as RGB isn't really the right way to model this.
                    // HSV would likely make this much easier

                    // try to create a new colour with the same intensity as the light source (as much as possible), but
                    // tint the shade based on the colour it's passing through
                    let light_intensity = light.red() + light.blue() + light.green();
                    let transmitted_colour = Colour::new(
                        hit_colour.red_factor(),
                        hit_colour.green_factor(),
                        hit_colour.blue_factor(),
                    ) * light_intensity;
                    let transmitted_colour = transmitted_colour.normalised();
                    // mix the colours together so that e.g. a red glass pane doesn't make the surface behind it totally red
                    // note: this may cause transparent materials to effectively emit light as e.g. a red light passing through a green plane will become slightly green
                    let tint = self.settings.transparent_colour_tint;
                    let colour = transmitted_colour * tint + light * (1.0 - tint);

                    colour * hit.with.material.transparency
                } else {
                    // object doesn't affect shadow calculations
                    light
                }
            })
    }
}
