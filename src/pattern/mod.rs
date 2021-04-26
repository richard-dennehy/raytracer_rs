use crate::pattern::Kind::{Checkers, Gradient, Ring, Solid, Striped};
use crate::{Colour, Point3D, Transform};

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
    Gradient { from: Colour, delta: Colour },
    Ring(Colour, Colour),
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

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn colour_at(&self, object_point: Point3D) -> Colour {
        use Kind::*;

        let inverse = self.transform.inverse();

        let (x, y, z, _) = inverse * object_point;

        // Adjust very small fractions such that when floored, they effectively round to the nearest integer, rather than rounding down.
        // This prevents acne caused by floating point errors (e.g. `-f64::EPSILON` should ideally floor to 0.0, rather than -1.0)
        let nudge = |f: f64| {
            let delta = f.ceil() - f;

            if delta != 0.0 && delta <= (f32::EPSILON as f64) {
                f + (f32::EPSILON as f64)
            } else {
                f
            }
        };

        let (x, y, z) = (nudge(x), nudge(y), nudge(z));

        match self.kind {
            Solid(colour) => colour,
            Striped(primary, _) if x.floor() % 2.0 == 0.0 => primary,
            Striped(_, secondary) => secondary,
            Gradient { from, delta } => from + delta * object_point.x().fract(),
            Ring(primary, _) if (x.powi(2) + z.powi(2)).sqrt().floor() % 2.0 == 0.0 => primary,
            Ring(_, secondary) => secondary,
            Checkers(primary, _) if (x.floor() + y.floor() + z.floor()) % 2.0 == 0.0 => primary,
            Checkers(_, secondary) => secondary,
        }
    }
}
