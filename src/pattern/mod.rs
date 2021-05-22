use crate::pattern::Kind::{Checkers, Gradient, Ring, Solid, Striped};
use crate::{Colour, Point3D, Transform};
use image::RgbImage;
use std::num::NonZeroUsize;
use std::sync::Arc;

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

#[derive(Clone, Debug, PartialEq)]
pub struct UvPattern {
    kind: UvPatternKind,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum UvPatternKind {
    Checkers(Colour, Colour),
    AlignmentCheck {
        main: Colour,
        top_left: Colour,
        top_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
    },
    Image(Arc<RgbImage>),
    // TODO use e.g. Map<UvRange, UvPattern> instead
    Cubic {
        front: Box<UvPattern>,
        back: Box<UvPattern>,
        left: Box<UvPattern>,
        right: Box<UvPattern>,
        top: Box<UvPattern>,
        bottom: Box<UvPattern>,
    },
    CappedCylinder {
        sides: Box<UvPattern>,
        top: Box<UvPattern>,
        bottom: Box<UvPattern>,
    },
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
        }
    }
}

impl UvPattern {
    pub fn checkers(
        primary: Colour,
        secondary: Colour,
        width: NonZeroUsize,
        height: NonZeroUsize,
    ) -> Self {
        UvPattern {
            kind: UvPatternKind::Checkers(primary, secondary),
            width: width.get(),
            height: height.get(),
        }
    }

    pub fn cubic(
        front: UvPattern,
        back: UvPattern,
        left: UvPattern,
        right: UvPattern,
        top: UvPattern,
        bottom: UvPattern,
    ) -> Self {
        UvPattern {
            kind: UvPatternKind::Cubic {
                front: Box::new(front),
                back: Box::new(back),
                left: Box::new(left),
                right: Box::new(right),
                top: Box::new(top),
                bottom: Box::new(bottom),
            },
            width: 1,
            height: 1,
        }
    }

    pub fn capped_cylinder(sides: UvPattern, top: UvPattern, bottom: UvPattern) -> Self {
        UvPattern {
            kind: UvPatternKind::CappedCylinder {
                sides: Box::new(sides),
                top: Box::new(top),
                bottom: Box::new(bottom),
            },
            width: 1,
            height: 1,
        }
    }

    pub fn alignment_check(
        main: Colour,
        top_left: Colour,
        top_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
    ) -> Self {
        UvPattern {
            kind: UvPatternKind::AlignmentCheck {
                main,
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            },
            width: 1,
            height: 1,
        }
    }

    pub fn image(img: Arc<RgbImage>) -> Self {
        UvPattern {
            kind: UvPatternKind::Image(img),
            width: 1,
            height: 1,
        }
    }
}

impl UvPattern {
    pub fn colour_at(&self, (u, v): (f64, f64)) -> Colour {
        let u = nudge(u) * self.width as f64;
        let v = nudge(v) * self.height as f64;

        match &self.kind {
            UvPatternKind::Checkers(primary, _)
                if (u.floor() + v.floor()) % 2.0 <= f64::EPSILON =>
            {
                *primary
            }
            UvPatternKind::Checkers(_, secondary) => *secondary,
            UvPatternKind::AlignmentCheck { top_left, .. }
                if (u.fract() - 0.2) <= f64::EPSILON && (v.fract() + 0.2) >= 1.0 =>
            {
                *top_left
            }
            UvPatternKind::AlignmentCheck { top_right, .. }
                if (u.fract() + 0.2) >= 1.0 && (v.fract() + 0.2) >= 1.0 =>
            {
                *top_right
            }
            UvPatternKind::AlignmentCheck { bottom_left, .. }
                if (u.fract() - 0.2) <= f64::EPSILON && (v.fract() - 0.2) <= f64::EPSILON =>
            {
                *bottom_left
            }
            UvPatternKind::AlignmentCheck { bottom_right, .. }
                if (u.fract() + 0.2) >= 1.0 && (v.fract() - 0.2) <= f64::EPSILON =>
            {
                *bottom_right
            }
            UvPatternKind::AlignmentCheck { main, .. } => *main,
            UvPatternKind::Image(img) => {
                let v = 1.0 - v;

                let x = u * (img.width() - 1) as f64;
                let y = v * (img.height() - 1) as f64;

                let pixel = img.get_pixel(x.round() as _, y.round() as _);
                Colour::new(
                    pixel.0[0] as f64 / 255.0,
                    pixel.0[1] as f64 / 255.0,
                    pixel.0[2] as f64 / 255.0,
                )
            }
            UvPatternKind::Cubic { .. } if u < 0.0 || v < 0.0 || u > 3.0 || v > 4.0 => {
                panic!("UV coordinates out of bounds for Cubic UV Map {:?}", (u, v))
            }
            UvPatternKind::Cubic { front, .. } if u <= 1.0 && v >= 2.0 && v <= 3.0 => {
                front.colour_at((u, v - 2.0))
            }
            UvPatternKind::Cubic { .. } if u <= 1.0 => {
                panic!("UV coordinates out of bounds for Cubic UV Map {:?}", (u, v))
            }
            UvPatternKind::Cubic { left, .. } if u <= 2.0 && v >= 3.0 => {
                left.colour_at((u - 1.0, v - 3.0))
            }
            UvPatternKind::Cubic { bottom, .. } if u <= 2.0 && v >= 2.0 => {
                bottom.colour_at((u - 1.0, v - 2.0))
            }
            UvPatternKind::Cubic { right, .. } if u <= 2.0 && v >= 1.0 => {
                right.colour_at((u - 1.0, v - 1.0))
            }
            UvPatternKind::Cubic { top, .. } if u <= 2.0 => top.colour_at((u - 1.0, v)),
            UvPatternKind::Cubic { back, .. } if u >= 2.0 && v <= 3.0 && v >= 2.0 => {
                back.colour_at((u - 2.0, v - 2.0))
            }
            UvPatternKind::Cubic { .. } => {
                panic!("UV coordinates out of bounds for Cubic UV Map {:?}", (u, v))
            }
            UvPatternKind::CappedCylinder { sides, .. } if u <= 1.0 && v <= 1.0 => {
                sides.colour_at((u, v))
            }
            UvPatternKind::CappedCylinder { top, .. } if u <= 2.0 => top.colour_at((u - 1.0, v)),
            UvPatternKind::CappedCylinder { bottom, .. } if u <= 3.0 => {
                bottom.colour_at((u - 2.0, v))
            }
            UvPatternKind::CappedCylinder { .. } => {
                panic!(
                    "UV coordinates out of bounds for Capped Cylinder UV Map {:?}",
                    (u, v)
                )
            }
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
