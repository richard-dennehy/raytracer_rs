use crate::{Colour, Point3D};

pub struct PointLight {
    pub intensity: Colour,
    pub position: Point3D,
}

impl PointLight {
    pub fn new(intensity: Colour, position: Point3D) -> Self {
        PointLight {
            intensity,
            position,
        }
    }
}
