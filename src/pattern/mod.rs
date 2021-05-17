use crate::pattern::Kind::{Checkers, CubicTexture, Gradient, Ring, Solid, Striped, Texture};
use crate::{Colour, Point3D, Transform, Vector};
use image::RgbImage;
use std::f64::consts::PI;
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
    CubicTexture {
        left: UvPattern,
        right: UvPattern,
        front: UvPattern,
        back: UvPattern,
        top: UvPattern,
        bottom: UvPattern,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum UvMap {
    Spherical,
    Planar,
    Cylindrical,
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
    Cubic {
        front: Box<UvPattern>,
        back: Box<UvPattern>,
        left: Box<UvPattern>,
        right: Box<UvPattern>,
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

    pub fn texture(uv_pattern: UvPattern, uv_map: UvMap) -> Self {
        Pattern {
            kind: Texture { uv_map, uv_pattern },
            transform: Transform::identity(),
        }
    }

    pub fn cubic_texture(
        left: UvPattern,
        right: UvPattern,
        front: UvPattern,
        back: UvPattern,
        top: UvPattern,
        bottom: UvPattern,
    ) -> Self {
        Pattern {
            kind: CubicTexture {
                left,
                right,
                front,
                back,
                top,
                bottom,
            },
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
            CubicTexture {
                left,
                right,
                front,
                back,
                top,
                bottom,
            } => cubic_colour_at(Point3D::new(x, y, z), left, right, front, back, top, bottom),
        }
    }
}

fn cubic_colour_at(
    point: Point3D,
    left: &UvPattern,
    right: &UvPattern,
    front: &UvPattern,
    back: &UvPattern,
    top: &UvPattern,
    bottom: &UvPattern,
) -> Colour {
    let largest = point.x().abs().max(point.y().abs().max(point.z().abs()));

    if largest == point.x() {
        let u = (1.0 - point.z()).rem_euclid(2.0) / 2.0;
        let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

        right.colour_at((u, v))
    } else if largest == -point.x() {
        let u = (1.0 + point.z()).rem_euclid(2.0) / 2.0;
        let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

        left.colour_at((u, v))
    } else if largest == point.y() {
        let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
        let v = (1.0 - point.z()).rem_euclid(2.0) / 2.0;

        top.colour_at((u, v))
    } else if largest == -point.y() {
        let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
        let v = (1.0 + point.z()).rem_euclid(2.0) / 2.0;

        bottom.colour_at((u, v))
    } else if largest == point.z() {
        let u = (1.0 + point.x()).rem_euclid(2.0) / 2.0;
        let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

        front.colour_at((u, v))
    } else {
        let u = (1.0 - point.x()).rem_euclid(2.0) / 2.0;
        let v = (1.0 + point.y()).rem_euclid(2.0) / 2.0;

        back.colour_at((u, v))
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
            UvMap::Planar => (point.x().rem_euclid(1.0), point.z().rem_euclid(1.0)),
            UvMap::Cylindrical => {
                // FIXME doesn't work properly on the caps
                // similar to spherical map on the sides

                // azimuthal angle
                let theta = point.x().atan2(point.z());
                let raw_u = theta / (2.0 * PI);
                // corrects backwards azimuthal angle
                let u = 1.0 - (raw_u + 0.5);

                let v = point.y().rem_euclid(1.0);
                (u, v)
            }
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
