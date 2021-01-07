use crate::pattern::Kind::{Checkers, Gradient, Ring, Solid, Striped};
use crate::{Colour, Matrix4D, Point3D};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    kind: Kind,
    transform: Matrix4D,
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
            transform: Matrix4D::identity(),
        }
    }

    pub const fn striped(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Striped(primary, secondary),
            transform: Matrix4D::identity(),
        }
    }

    pub fn gradient(from: Colour, to: Colour) -> Self {
        Pattern {
            kind: Gradient {
                from,
                delta: to - from,
            },
            transform: Matrix4D::identity(),
        }
    }

    pub const fn ring(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Ring(primary, secondary),
            transform: Matrix4D::identity(),
        }
    }

    pub const fn checkers(primary: Colour, secondary: Colour) -> Self {
        Pattern {
            kind: Checkers(primary, secondary),
            transform: Matrix4D::identity(),
        }
    }

    pub fn with_transform(mut self, transform: Matrix4D) -> Self {
        self.transform = transform;
        self
    }

    pub fn colour_at(&self, object_point: Point3D) -> Colour {
        use Kind::*;

        let inverse = self
            .transform
            .inverse()
            .expect("A transformation matrix must be invertible");

        let (x, y, z, _) = inverse * object_point;
        let point = Point3D::new(x, y, z);

        match self.kind {
            Solid(colour) => colour,
            Striped(primary, _) if point.x().floor() % 2.0 == 0.0 => primary,
            Striped(_, secondary) => secondary,
            Gradient { from, delta } => from + delta * object_point.x().fract(),
            Ring(primary, _)
                if (point.x() * point.x() + point.z() * point.z())
                    .sqrt()
                    .floor()
                    % 2.0
                    == 0.0 =>
            {
                primary
            }
            Ring(_, secondary) => secondary,
            Checkers(primary, _) if (x.floor() + y.floor() + z.floor()) % 2.0 == 0.0 => primary,
            Checkers(_, secondary) => secondary,
        }
    }
}
