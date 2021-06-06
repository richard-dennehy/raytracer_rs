use crate::{Colour, Point3D, Vector3D};
use itertools::Itertools;
use std::num::NonZeroU8;

#[derive(Debug, PartialEq, Clone)]
pub struct Light {
    samples: Vec<LightSample>,
}

impl Light {
    pub fn point(colour: Colour, position: Point3D) -> Self {
        Light {
            samples: vec![LightSample::new(position, colour)],
        }
    }

    pub fn area(
        colour: Colour,
        bottom_left: Point3D,
        u: Vector3D,
        v: Vector3D,
        u_steps: NonZeroU8,
        v_steps: NonZeroU8,
    ) -> Self {
        let cell_u = u / (u_steps.get() as f64);
        let cell_v = v / (v_steps.get() as f64);

        let offset = bottom_left + cell_u / 2.0 + cell_v / 2.0;

        let samples = (0..u_steps.get())
            .cartesian_product(0..v_steps.get())
            .map(|(u, v)| LightSample::new(offset + cell_u * u as f64 + cell_v * v as f64, colour))
            .collect();

        Light { samples }
    }

    // FIXME if `world` uses running average this doesn't need to return the len
    pub fn samples(&self) -> (impl Iterator<Item = &LightSample>, usize) {
        (self.samples.iter(), self.samples.len())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LightSample {
    pub position: Point3D,
    pub colour: Colour,
}

impl LightSample {
    pub fn new(position: Point3D, colour: Colour) -> Self {
        LightSample { position, colour }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampling_an_area_light_should_take_one_sample_from_each_cell() {
        let area = Light::area(
            Colour::WHITE,
            Point3D::ORIGIN,
            Vector3D::new(2.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            nonzero_ext::nonzero!(4u8),
            nonzero_ext::nonzero!(2u8),
        );

        let mut samples = area.samples().0;
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.25, 0.0, 0.25)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.25, 0.0, 0.75)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.75, 0.0, 0.25)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.75, 0.0, 0.75)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.25, 0.0, 0.25)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.25, 0.0, 0.75)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.75, 0.0, 0.25)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.75, 0.0, 0.75)
        );
        assert!(samples.next().is_none());
    }
}
