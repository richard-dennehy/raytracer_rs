use crate::{Colour, Point3D};
use std::ops::Mul;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Light {
    Point { colour: Colour, position: Point3D },
}

impl Light {
    pub fn point(colour: Colour, position: Point3D) -> Self {
        Light::Point { colour, position }
    }

    pub fn colour(&self) -> Colour {
        match &self {
            Light::Point { colour, .. } => *colour,
        }
    }

    pub fn position(&self) -> Point3D {
        match &self {
            Light::Point { position, .. } => *position,
        }
    }
}

impl Mul<f64> for Light {
    type Output = Light;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Light::Point { colour, position } => Light::Point {
                colour: colour * rhs,
                position,
            },
        }
    }
}
