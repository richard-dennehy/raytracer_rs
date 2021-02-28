use crate::{Camera, Canvas, World};
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    let colours = (0..camera.width().get())
        .into_par_iter()
        .flat_map(|x| {
            (0..camera.height().get())
                .into_par_iter()
                .map(move |y| (x, y))
        })
        .map(|(x, y)| {
            let ray = camera.ray_at(x, y);
            let colour = world.colour_at(ray);
            (x, y, colour)
        })
        .collect::<Vec<_>>();

    colours
        .into_iter()
        .for_each(|(x, y, colour)| canvas.set(x, y, colour));

    canvas
}
