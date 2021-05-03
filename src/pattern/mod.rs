use crate::pattern::Kind::{Checkers, Gradient, Ring, Solid, Striped, Texture};
use crate::{Colour, Point3D, Transform, Vector};
use std::f64::consts::PI;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    kind: Kind,
    transform: Transform,
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Solid(Colour),
    Striped(Colour, Colour),
    Gradient {
        from: Colour,
        delta: Colour,
    },
    Ring(Colour, Colour),
    Checkers(Colour, Colour),
    Texture {
        uv_map: UvMap,
        uv_pattern: UvPattern,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum UvMap {
    Spherical,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UvPattern {
    kind: UvPatternKind,
    width: f64,
    height: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum UvPatternKind {
    Checkers(Colour, Colour),
}

impl Pattern {
    pub const fn solid(colour: Colour) -> Self {
        Pattern {
            kind: Solid(colour),
            transform: Transform::identity(),
        }
    }

    pub const fn striped(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Striped(primary, secondary),
            transform: Transform::identity(),
        }
    }

    pub fn gradient(from: Colour, to: Colour) -> Self {
        Pattern {
            kind: Gradient {
                from,
                delta: to - from,
            },
            transform: Transform::identity(),
        }
    }

    pub const fn ring(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Ring(primary, secondary),
            transform: Transform::identity(),
        }
    }

    pub const fn checkers(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Checkers(primary, secondary),
            transform: Transform::identity(),
        }
    }

    pub fn texture(uv_pattern: UvPattern, uv_map: UvMap) -> Self {
        Pattern {
            kind: Texture { uv_map, uv_pattern },
            transform: Transform::identity(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn colour_at(&self, object_point: Point3D) -> Colour {
        use Kind::*;

        let inverse = self.transform.inverse();

        let (x, y, z, _) = inverse * object_point;

        let (x, y, z) = (nudge(x), nudge(y), nudge(z));

        match &self.kind {
            Solid(colour) => *colour,
            Striped(primary, _) if x.floor() % 2.0 == 0.0 => *primary,
            Striped(_, secondary) => *secondary,
            Gradient { from, delta } => from + &(delta * object_point.x().fract()),
            Ring(primary, _) if (x.powi(2) + z.powi(2)).sqrt().floor() % 2.0 == 0.0 => *primary,
            Ring(_, secondary) => *secondary,
            Checkers(primary, _) if (x.floor() + y.floor() + z.floor()) % 2.0 == 0.0 => *primary,
            Checkers(_, secondary) => *secondary,
            Texture { uv_map, uv_pattern } => {
                uv_pattern.colour_at(uv_map.uv(Point3D::new(x, y, z)))
            }
        }
    }
}

impl UvPattern {
    pub fn checkers(primary: Colour, secondary: Colour) -> Self {
        UvPattern {
            kind: UvPatternKind::Checkers(primary, secondary),
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
}

impl UvMap {
    fn uv(&self, point: Point3D) -> (f64, f64) {
        match self {
            UvMap::Spherical => {
                // See https://en.wikipedia.org/wiki/Spherical_coordinate_system noting this uses _mathematical_ notation

                // azimuthal angle - this is backwards but gets corrected later
                let theta = point.x().atan2(point.z());
                // given the centre is at the world origin, the radius is given by the magnitude of the vector
                // from the world origin to the point
                let r = (point - Point3D::ORIGIN).magnitude();
                // polar angle
                let phi = (point.y() / r).acos();
                let raw_u = theta / (2.0 * PI);
                // corrects backwards azimuthal angle
                let u = 1.0 - (raw_u + 0.5);
                // subtract from 1 to invert `v` such that 1 is the northernmost point
                let v = 1.0 - (phi / PI);

                (u, v)
            }
        }
    }
}

impl UvPattern {
    fn colour_at(&self, (u, v): (f64, f64)) -> Colour {
        let u = (nudge(u) * self.width).floor();
        let v = (nudge(v) * self.height).floor();

        match self.kind {
            UvPatternKind::Checkers(primary, _) if (u + v) % 2.0 <= f64::EPSILON => primary,
            UvPatternKind::Checkers(_, secondary) => secondary,
        }
    }
}

/// Adjust very small fractions such that when floored, they effectively round to the nearest integer, rather than rounding down.
/// This prevents acne caused by floating point errors (e.g. `-f64::EPSILON` should ideally floor to 0.0, rather than -1.0)
fn nudge(f: f64) -> f64 {
    let delta = f.ceil() - f;

    if delta != 0.0 && delta <= (f32::EPSILON as f64) {
        f + (f32::EPSILON as f64)
    } else {
        f
    }
}
