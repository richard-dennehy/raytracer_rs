use crate::{Camera, Canvas, World};
use rand::Rng;
use std::num::NonZeroU8;

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera, samples: NonZeroU8) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    canvas.draw(|x, y| {
        let centre_colour = world.colour_at(camera.ray_at(x, y, 0.5, 0.5));

        // faster to generate a single `rng` instance as opposed to once per sample
        let mut rng = rand::thread_rng();
        let mut random_offset = || rng.gen::<f64>();

        (0..samples.get() - 1)
            .into_iter()
            .fold(centre_colour, |acc, _| {
                let sample = world.colour_at(camera.ray_at(x, y, random_offset(), random_offset()));
                acc.average(sample)
            })
    });

    canvas
}
