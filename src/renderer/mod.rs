use crate::{Camera, Canvas, World};

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    canvas.draw(|x, y| world.colour_at(camera.ray_at(x, y)));

    canvas
}
