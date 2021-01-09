use crate::{Colour, Matrix4D, Object, Point3D, PointLight, Vector3D};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point3D {
        self.origin + self.direction * time
    }

    pub fn transformed(&self, transformation: &Matrix4D) -> Self {
        let (x, y, z, w) = transformation * self.origin;
        debug_assert!(w == 1.0, "matrix transform did not return a Point");
        let transformed_origin = Point3D::new(x, y, z);

        let (x, y, z, w) = transformation * self.direction;
        debug_assert!(w == 0.0, "matrix transform did not return a Vector");
        let transformed_direction = Vector3D::new(x, y, z);

        Ray::new(transformed_origin, transformed_direction)
    }

    pub fn hit_data<'obj>(&self, intersection: Intersection<'obj>) -> HitData<'obj> {
        let point = self.position(intersection.t);
        let eye = -self.direction;
        let normal = intersection.with.normal_at(point);

        let inside = normal.dot(&eye) < 0.0;
        let offset_point = point + normal * (f32::EPSILON as f64); // f64 epsilon isn't sufficient to compensate for rounding errors

        let normal = if inside { -normal } else { normal };
        let reflection = self.direction.reflect_through(normal);

        HitData {
            t: intersection.t,
            object: intersection.with,
            point,
            eye,
            normal,
            inside,
            offset_point,
            reflection,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'with> {
    pub t: f64,
    pub with: &'with Object,
}

impl<'with> Intersection<'with> {
    pub fn new(t: f64, with: &'with Object) -> Intersection {
        Intersection { t, with }
    }
}

pub struct HitData<'obj> {
    pub t: f64,
    pub object: &'obj Object,
    pub point: Point3D,
    pub eye: Vector3D,
    pub normal: Vector3D,
    pub inside: bool,
    pub offset_point: Point3D,
    pub reflection: Vector3D,
}

impl<'obj> HitData<'obj> {
    pub fn colour(&self, light: &PointLight, in_shadow: bool) -> Colour {
        self.object
            .colour_at(self.point, light, self.eye, in_shadow)
    }
}

/// Invariants:
///  - always sorted by ascending `t` values
#[derive(Debug, PartialEq)]
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

    pub fn hit(&self) -> Option<Intersection<'scene>> {
        self.0.iter().find(|&intersect| intersect.t >= 0.0).cloned()
    }

    pub fn append(&mut self, mut other: Intersections<'scene>) {
        self.0.append(&mut other.0);
        self.sort();
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.0.get(index)
    }

    pub fn sort(&mut self) {
        self.0.sort_unstable_by(|first, second| {
            f64::partial_cmp(&first.t, &second.t).expect("a `t` value should never be NaN")
        })
    }
}
