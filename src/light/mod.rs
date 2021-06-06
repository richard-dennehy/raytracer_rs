use crate::{Colour, Point3D, Vector3D};
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
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
        seed: u64,
    ) -> Self {
        let cell_u = u / (u_steps.get() as f64);
        let cell_v = v / (v_steps.get() as f64);

        let mut rng = StdRng::seed_from_u64(seed);

        // This reduces the obvious banding from using a constant offset, but the banding is still visible.
        // The issue is that this needs to be deterministic, so shadows don't move around when rerendering the same image,
        // but not _look_ deterministic, e.g. the shadows should blend smoothly into each other.
        // Producing deterministic "random" data is very challenging in a multi-threaded renderer, as
        // the order the threads execute in is effectively random. It may be possible to move the random offset
        // into `world` and use e.g. the target Point as a seed, but this would force the threads to synchronise
        // with each other, and substantially impact performance
        let mut offset = || bottom_left + cell_u * rng.gen::<f64>() + cell_v * rng.gen::<f64>();

        let samples = (0..u_steps.get())
            .cartesian_product(0..v_steps.get())
            .map(|(u, v)| {
                LightSample::new(offset() + cell_u * u as f64 + cell_v * v as f64, colour)
            })
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
            0,
        );

        let mut samples = area.samples().0;
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.3655567079318523, 0.0, 0.3867300921766191)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.012922317116677517, 0.0, 0.7920796309681001)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.6310853162862626, 0.0, 0.38599345287341036)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(0.6093484553357547, 0.0, 0.8968567283345681)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.3746692145496366, 0.0, 0.48446887400119504)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.4981586781621425, 0.0, 0.5696882153025913)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.556432499904909, 0.0, 0.4824178498227347)
        );
        assert_eq!(
            samples.next().unwrap().position,
            Point3D::new(1.5247000272159905, 0.0, 0.7128510331304977)
        );
        assert!(samples.next().is_none());
    }
}
