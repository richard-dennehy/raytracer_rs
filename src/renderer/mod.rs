use crate::{Camera, Canvas, World};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera, subsamples: Subsamples) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    canvas.draw(|x, y| {
        let centre_colour = world.colour_at(camera.ray_at(x, y, 0.5, 0.5));

        subsamples
            .offsets()
            .fold(centre_colour, |acc, (x_offset, y_offset)| {
                let sample = world.colour_at(camera.ray_at(x, y, *x_offset, *y_offset));
                acc.average(sample)
            })
    });

    canvas
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Subsamples {
    /// only exact centre of pixel
    None,
    /// centre and four corners
    X4,
    /// centre, corners, and mid-points between corners (the centre of each edge)
    X8,
    /// centre, corners, edge mid-points, and halfway point between each point and the centre
    X16,
}

impl Subsamples {
    const TOP_LEFT: (f64, f64) = (0.0, 0.0);
    const TOP_MIDDLE: (f64, f64) = (0.5, 0.0);
    const TOP_RIGHT: (f64, f64) = (1.0, 0.0);

    const BOTTOM_LEFT: (f64, f64) = (0.0, 1.0);
    const BOTTOM_MIDDLE: (f64, f64) = (0.5, 1.0);
    const BOTTOM_RIGHT: (f64, f64) = (1.0, 1.0);

    const LEFT_MIDDLE: (f64, f64) = (0.0, 0.5);
    const RIGHT_MIDDLE: (f64, f64) = (1.0, 0.5);

    const TOP_LEFT_HALFWAY: (f64, f64) = (0.25, 0.25);
    const TOP_MIDDLE_HALFWAY: (f64, f64) = (0.5, 0.25);
    const TOP_RIGHT_HALFWAY: (f64, f64) = (0.75, 0.25);

    const BOTTOM_LEFT_HALFWAY: (f64, f64) = (0.25, 0.75);
    const BOTTOM_MIDDLE_HALFWAY: (f64, f64) = (0.5, 0.75);
    const BOTTOM_RIGHT_HALFWAY: (f64, f64) = (0.75, 0.75);

    const LEFT_MIDDLE_HALFWAY: (f64, f64) = (0.25, 0.5);
    const RIGHT_MIDDLE_HALFWAY: (f64, f64) = (0.75, 0.5);

    pub fn offsets(&self) -> Iter<(f64, f64)> {
        match self {
            Subsamples::None => [].iter(),
            Subsamples::X4 => [
                Self::TOP_LEFT,
                Self::TOP_RIGHT,
                Self::BOTTOM_LEFT,
                Self::BOTTOM_RIGHT,
            ]
            .iter(),
            Subsamples::X8 => [
                Self::TOP_LEFT,
                Self::TOP_MIDDLE,
                Self::TOP_RIGHT,
                Self::BOTTOM_LEFT,
                Self::BOTTOM_MIDDLE,
                Self::BOTTOM_RIGHT,
                Self::LEFT_MIDDLE,
                Self::RIGHT_MIDDLE,
            ]
            .iter(),
            Subsamples::X16 => [
                Self::TOP_LEFT,
                Self::TOP_MIDDLE,
                Self::TOP_RIGHT,
                Self::BOTTOM_LEFT,
                Self::BOTTOM_MIDDLE,
                Self::BOTTOM_RIGHT,
                Self::LEFT_MIDDLE,
                Self::RIGHT_MIDDLE,
                Self::TOP_LEFT_HALFWAY,
                Self::TOP_MIDDLE_HALFWAY,
                Self::TOP_RIGHT_HALFWAY,
                Self::BOTTOM_LEFT_HALFWAY,
                Self::BOTTOM_MIDDLE_HALFWAY,
                Self::BOTTOM_RIGHT_HALFWAY,
                Self::LEFT_MIDDLE_HALFWAY,
                Self::RIGHT_MIDDLE_HALFWAY,
            ]
            .iter(),
        }
    }
}

impl Display for Subsamples {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Subsamples::None => "none",
                Subsamples::X4 => "X4",
                Subsamples::X8 => "X8",
                Subsamples::X16 => "X16",
            }
        )
    }
}

struct Samples {
    offsets: Vec<(f64, f64)>,
}

impl Samples {
    pub fn new(grid_size: u8) -> Self {
        let initial = 1.0 / (grid_size * 2) as f64;
        let increment = 1.0 / grid_size as f64;

        let offsets = (0..grid_size)
            .flat_map(|y| (0..grid_size).map(move |x| (x, y)))
            .map(|(x, y)| {
                (
                    initial + (x as f64) * increment,
                    initial + (y as f64) * increment,
                )
            })
            .collect();

        Self { offsets }
    }

    fn offsets(&self) -> Iter<(f64, f64)> {
        self.offsets.iter()
    }
}
