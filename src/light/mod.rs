use crate::{Colour, Point3D, Vector3D};
use itertools::Itertools;
use std::num::NonZeroU8;

#[derive(Debug, PartialEq, Clone)]
pub struct Light {
    inner: inner::Light,
}

impl Light {
    pub fn point(colour: Colour, position: Point3D) -> Self {
        Light {
            inner: inner::Light::Point { colour, position },
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
        let cell_dimensions = CellDimensions {
            u: u / (u_steps.get() as f64),
            v: v / (v_steps.get() as f64),
        };

        Light {
            inner: inner::Light::Area {
                colour,
                corner: bottom_left,
                cell_dimensions,
                width: u_steps.get(),
                height: v_steps.get(),
            },
        }
    }

    pub fn colour(&self) -> Colour {
        match &self.inner {
            inner::Light::Point { colour, .. } => *colour,
            inner::Light::Area { colour, .. } => *colour,
        }
    }

    // FIXME performance: calculate this once and hand out references
    pub fn samples(&self) -> Vec<LightSample> {
        match &self.inner {
            inner::Light::Point { position, colour } => vec![LightSample::new(*position, *colour)],
            inner::Light::Area {
                corner,
                cell_dimensions,
                width,
                height,
                colour,
                ..
            } => {
                let offset = *corner + cell_dimensions.u / 2.0 + cell_dimensions.v / 2.0;

                (0..*width)
                    .cartesian_product(0..*height)
                    .map(|(u, v)| {
                        LightSample::new(
                            offset + cell_dimensions.u * u as f64 + cell_dimensions.v * v as f64,
                            *colour,
                        )
                    })
                    .collect()
            }
        }
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

#[derive(Debug, PartialEq, Clone)]
struct CellDimensions {
    u: Vector3D,
    v: Vector3D,
}

mod inner {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub(super) enum Light {
        Point {
            colour: Colour,
            position: Point3D,
        },
        Area {
            colour: Colour,
            corner: Point3D,
            cell_dimensions: CellDimensions,
            width: u8,
            height: u8,
        },
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

        let mut samples = area.samples().into_iter();
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
