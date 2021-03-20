use crate::matrix::Matrix4D;
use crate::{Colour, Light, Normal3D, Object, Point3D, Vector, Vector3D};
use std::vec::IntoIter;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    // a ray should be normalised when created, but may be stretched or squashed
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Normal3D) -> Self {
        Ray {
            origin,
            direction: direction.into(),
        }
    }

    pub fn position(&self, time: f64) -> Point3D {
        self.origin + self.direction * time
    }

    pub fn transformed(&self, transformation: &Matrix4D) -> Self {
        let (x, y, z, _) = transformation * self.origin;
        let origin = Point3D::new(x, y, z);

        let (x, y, z, _) = transformation * self.direction;
        let direction = Vector3D::new(x, y, z);

        Ray { origin, direction }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection<'with> {
    pub t: f64,
    pub with: &'with Object,
    pub uv: Option<(f64, f64)>,
}

impl<'with> Intersection<'with> {
    pub fn new(t: f64, with: &'with Object) -> Intersection {
        Intersection { t, with, uv: None }
    }

    pub fn with_uv(t: f64, with: &'with Object, u: f64, v: f64) -> Intersection {
        Intersection {
            t,
            with,
            uv: Some((u, v)),
        }
    }
}

pub struct HitData<'obj> {
    pub object: &'obj Object,
    pub eye: Normal3D,
    pub normal: Normal3D,
    pub over_point: Point3D,
    pub under_point: Point3D,
    pub entered_refractive: f64,
    pub exited_refractive: f64,
}

impl<'obj> HitData<'obj> {
    pub fn from(
        ray: &Ray,
        intersection: Intersection<'obj>,
        intersections: Intersections<'obj>,
    ) -> Self {
        let point = ray.position(intersection.t);
        let eye = -ray.direction.normalised();
        let normal = intersection.with.normal_at(point, intersection.uv);

        let inside = normal.dot(eye) < 0.0;

        let normal = if inside { -normal } else { normal };
        let offset = normal * (f32::EPSILON as f64); // f64 epsilon isn't sufficient to compensate for rounding errors
        let over_point = point + offset;
        let under_point = point - offset;

        // calculate refraction changes from entering one material and exiting another (including the empty space)
        let mut entered_refractive = 1.0;
        let mut exited_refractive = 1.0;
        let mut containers: Vec<&Object> = vec![];

        for i in intersections.0.iter() {
            if i.t == intersection.t && i.with.id() == intersection.with.id() {
                // intersection from entering object
                if let Some(&last) = containers.last() {
                    entered_refractive = last.material.refractive;
                }
            }

            if let Some(index) = containers
                .iter()
                .cloned()
                .enumerate()
                .find(|(_, obj)| obj.id() == i.with.id())
                .map(|(idx, _)| idx)
            {
                containers.remove(index); // exiting transparent object
            } else {
                containers.push(i.with); // entering transparent object
            }

            if i.t == intersection.t && i.with.id() == intersection.with.id() {
                // intersection from exiting object
                if let Some(&last) = containers.last() {
                    exited_refractive = last.material.refractive;
                    break;
                }
            }
        }

        HitData {
            object: intersection.with,
            eye,
            normal,
            over_point,
            under_point,
            entered_refractive,
            exited_refractive,
        }
    }

    pub fn colour(&self, light: &Light, in_shadow: bool) -> Colour {
        self.object
            .colour_at(self.over_point, light, self.eye, self.normal, in_shadow)
    }

    pub fn reflection(&self) -> ReflectionData {
        let ratio = self.entered_refractive / self.exited_refractive;
        let cos_i = self.eye.dot(self.normal);
        let sin2_t = ratio.powi(2) * (1.0 - cos_i.powi(2));

        ReflectionData {
            cos_i,
            ratio,
            sin2_t,
        }
    }
}

pub struct ReflectionData {
    pub cos_i: f64,
    pub ratio: f64,
    pub sin2_t: f64,
}

impl ReflectionData {
    pub fn is_total(&self) -> bool {
        self.sin2_t > 1.0
    }

    /// note: reflection must not be total (sin2_t must not be > 1.0)
    pub fn refraction_vector(&self, normal: Normal3D, eye: Normal3D) -> Vector3D {
        debug_assert!(self.sin2_t <= 1.0);

        normal * (self.ratio * self.cos_i - self.cos_t()) - (eye * self.ratio)
    }

    /// `shlick` approximation of fresnel
    pub fn reflectance(&self, entered_refractive: f64, exited_refractive: f64) -> f64 {
        if self.is_total() {
            return 1.0;
        }

        let cos = if entered_refractive > exited_refractive {
            self.cos_t()
        } else {
            self.cos_i
        };

        let r0 = ((entered_refractive - exited_refractive)
            / (entered_refractive + exited_refractive))
            .powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    /// note: reflection must not be total (sin2_t must not be > 1.0)
    fn cos_t(&self) -> f64 {
        debug_assert!(self.sin2_t <= 1.0);

        (1.0 - self.sin2_t).sqrt()
    }
}

/// Invariants:
///  - always sorted by ascending `t` values
#[derive(Clone, Debug)]
pub struct Intersections<'scene>(Vec<Intersection<'scene>>);

impl<'scene> Intersections<'scene> {
    #[cfg(test)]
    pub fn underlying(&self) -> &Vec<Intersection<'scene>> {
        &self.0
    }

    pub fn empty() -> Self {
        Intersections(Vec::new())
    }

    pub fn of(intersections: Vec<Intersection<'scene>>) -> Self {
        let mut this = Intersections(intersections);
        this.sort();
        this
    }

    pub fn push(&mut self, intersection: Intersection<'scene>) {
        self.0.push(intersection);
        self.sort();
    }

    pub fn join(mut self, mut other: Intersections<'scene>) -> Self {
        self.0.append(&mut other.0);
        self.sort();

        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn hit(&self) -> Option<Intersection<'scene>> {
        self.0.iter().find(|&intersect| intersect.t >= 0.0).cloned()
    }

    pub fn shadow_hit(&self) -> Option<Intersection<'scene>> {
        self.0
            .iter()
            .find(|&intersect| intersect.t >= 0.0 && intersect.with.material.casts_shadow)
            .cloned()
    }

    pub fn append(&mut self, mut other: Intersections<'scene>) {
        self.0.append(&mut other.0);
        self.sort();
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.0.get(index)
    }

    pub fn into_iter(self) -> IntoIter<Intersection<'scene>> {
        self.0.into_iter()
    }

    fn sort(&mut self) {
        self.0.sort_unstable_by(|first, second| {
            f64::partial_cmp(&first.t, &second.t).expect("a `t` value should never be NaN")
        })
    }
}
