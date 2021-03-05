use crate::{Camera, Canvas, World};
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    // TODO performance:
    //   investigate whether `x` parallel tasks that each cast `y` rays _sequentially_
    //   is faster than casting `x` * `y` rays in parallel
    //   (this would avoid having to collect into the initial vector, and may reduce stop/start overhead)

    let pixels = (0..camera.width().get())
        .into_iter()
        .flat_map(|x| (0..camera.height().get()).into_iter().map(move |y| (x, y)))
        // apparently can't call e.g. `into_par_iter` here, so have to collect into Vec, then par_iter that
        .collect::<Vec<_>>();

    let colours = pixels
        .into_par_iter()
        .map(|(x, y)| {
            let ray = camera.ray_at(x, y);
            let colour = world.colour_at(ray);
            (x, y, colour)
        })
        // need to join parallel iterators here, as the final step mutates the canvas
        // although it's strictly safe to mutate in parallel, that's very difficult to statically prove
        .collect::<Vec<_>>();

    colours
        .into_iter()
        .for_each(|(x, y, colour)| canvas.set(x, y, colour));

    canvas
}
