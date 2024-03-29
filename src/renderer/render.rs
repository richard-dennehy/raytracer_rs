use super::*;
use crate::core::Colour;
use crate::scene::World;
use smallvec::SmallVec;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
use std::slice::Iter;

/// # Parameters
/// `show_progress`: set to `true` when using e.g. `cargo run` for real-time progress updates;
///                  set to `false` when running benchmarks, otherwise it'll cripple performance due to stdout locking
pub fn render(world: &World, camera: &Camera, samples: &Samples, show_progress: bool) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    canvas.draw(show_progress, |x, y| {
        let mut corners = samples.corner_offsets();
        let (x_offset, y_offset) = corners.next().unwrap();
        let top_left = world.colour_at(camera.ray_at(x, y, *x_offset, *y_offset));

        let average_samples = |acc: Colour, (x_offset, y_offset): &(f64, f64)| {
            let sample = world.colour_at(camera.ray_at(x, y, *x_offset, *y_offset));
            acc.average(sample)
        };

        let corner_avg = corners.fold(top_left, average_samples);

        if samples.inner_samples() == 0 || corner_avg.is_similar_to(&top_left) {
            return top_left;
        }

        samples.inner_offsets().fold(corner_avg, average_samples)
    });

    canvas
}

#[derive(Debug, PartialEq)]
pub struct Samples {
    inner: Vec<(f64, f64)>,
    corners: SmallVec<[(f64, f64); 4]>,
}

impl Samples {
    pub fn single() -> Self {
        let mut corners = SmallVec::new();
        corners.push((0.5, 0.5));

        Self {
            inner: vec![],
            corners,
        }
    }

    pub fn grid(grid_size: NonZeroU8) -> Self {
        let grid_size = grid_size.get();

        if grid_size == 1 {
            return Self::single();
        }

        let initial = 1.0 / (grid_size * 2) as f64;
        let increment = 1.0 / grid_size as f64;

        let max = initial + (increment * (grid_size - 1) as f64);
        let corners = SmallVec::from([
            (initial, initial),
            (max, initial),
            (initial, max),
            (max, max),
        ]);

        let offsets = (0..grid_size)
            .flat_map(|y| (0..grid_size).map(move |x| (x, y)))
            // exclude corners
            .filter(|(x, y)| (*x != 0 && *x != grid_size - 1) || (*y != 0 && *y != grid_size - 1))
            .map(|(x, y)| {
                (
                    initial + (x as f64) * increment,
                    initial + (y as f64) * increment,
                )
            })
            .collect();

        Self {
            inner: offsets,
            corners,
        }
    }

    pub(super) fn inner_offsets(&self) -> Iter<(f64, f64)> {
        self.inner.iter()
    }

    pub(super) fn corner_offsets(&self) -> Iter<(f64, f64)> {
        self.corners.iter()
    }

    fn samples(&self) -> usize {
        self.inner.len() + self.corners.len()
    }

    fn inner_samples(&self) -> usize {
        self.inner.len()
    }
}

impl Display for Samples {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "X{}", self.samples())
    }
}
