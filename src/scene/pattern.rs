use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::sync::Arc;

use image::RgbImage;

use crate::core::Colour;
use crate::core::F64Ext;
use crate::core::Point3D;
use crate::core::Transform;

use super::pattern::Kind::{Checkers, Gradient, Ring, Striped};

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    kind: Kind,
    transform: Transform,
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Striped(Colour, Colour),
    Gradient { from: Colour, delta: Colour },
    Ring(Colour, Colour),
    Checkers(Colour, Colour),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UvPattern {
    kind: UvPatternKind,
    pub transform: Transform,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum UvPatternKind {
    Checkers {
        primary: Colour,
        secondary: Colour,
        width: usize,
        height: usize,
    },
    AlignmentCheck {
        main: Colour,
        top_left: Colour,
        top_right: Colour,
        bottom_left: Colour,
        bottom_right: Colour,
    },
    Image(Arc<RgbImage>),
    MultiFace(Vec<(RangeInclusive<f64>, RangeInclusive<f64>, UvPattern)>),
}

impl Pattern {
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
            kind: UvPatternKind::Checkers {
                primary,
                secondary,
                width: width.get(),
                height: height.get(),
            },
            transform: Transform::identity(),
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
            kind: UvPatternKind::MultiFace(vec![
                (1.0..=2.0, 0.0..=1.0, top),
                (1.0..=2.0, 1.0..=2.0, right),
                (0.0..=1.0, 2.0..=3.0, front),
                (1.0..=2.0, 2.0..=3.0, bottom),
                (2.0..=3.0, 2.0..=3.0, back),
                (1.0..=2.0, 3.0..=4.0, left),
            ]),
            transform: Transform::identity(),
        }
    }

    pub fn capped_cylinder(sides: UvPattern, top: UvPattern, bottom: UvPattern) -> Self {
        UvPattern {
            kind: UvPatternKind::MultiFace(vec![
                (0.0..=1.0, 0.0..=1.0, sides),
                (1.0..=2.0, 0.0..=1.0, top),
                (2.0..=3.0, 0.0..=1.0, bottom),
            ]),
            transform: Transform::identity(),
        }
    }

    /// intended for manually testing UV mapping on cubes - each corner should have the same colour
    /// on each face sharing that corner
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
            transform: Transform::identity(),
        }
    }

    pub fn image(img: Arc<RgbImage>) -> Self {
        UvPattern {
            kind: UvPatternKind::Image(img),
            transform: Transform::identity(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

impl UvPattern {
    pub fn colour_at(&self, (u, v): (f64, f64)) -> Colour {
        match &self.kind {
            UvPatternKind::Checkers {
                primary,
                secondary,
                width,
                height,
            } => {
                let u = nudge(u) * *width as f64;
                let v = nudge(v) * *height as f64;

                if (u.floor() + v.floor()) % 2.0 <= f64::EPSILON {
                    *primary
                } else {
                    *secondary
                }
            }
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

                let x = u.rem_euclid(1.0) * (img.width() - 1) as f64;
                let y = v.rem_euclid(1.0) * (img.height() - 1) as f64;

                let pixel = img.get_pixel(x.round() as _, y.round() as _);
                Colour::new(
                    pixel.0[0] as f64 / 255.0,
                    pixel.0[1] as f64 / 255.0,
                    pixel.0[2] as f64 / 255.0,
                )
            }
            UvPatternKind::MultiFace(faces) => faces
                .iter()
                .find_map(|(u_range, v_range, uv)| {
                    (u_range.contains(&u) && v_range.contains(&v)).then(|| uv.colour_at((u, v)))
                })
                .expect(&format!("UV coordinates out of bounds: {:?}", (u, v))),
        }
    }
}

/// Adjust very small fractions such that when floored, they effectively round to the nearest integer, rather than rounding down.
/// This prevents acne caused by floating point errors (e.g. `-f64::EPSILON` should ideally floor to 0.0, rather than -1.0)
fn nudge(f: f64) -> f64 {
    let delta = f.ceil() - f;

    if delta != 0.0 && delta.is_roughly_zero() {
        f + crate::core::EPSILON
    } else {
        f
    }
}
