use crate::{Colour, Point3D};

#[derive(Debug, PartialEq, Clone)]
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
