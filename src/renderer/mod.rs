use crate::{Camera, Canvas, World};

#[cfg(test)]
mod tests;

pub fn render(world: World, camera: Camera) -> Canvas {
    let mut canvas =
        Canvas::new(camera.width(), camera.height()).expect("Camera dimensions are too large");

    for x in 0..camera.width().get() {
        for y in 0..camera.height().get() {
            let ray = camera.ray_at(x, y);
            let colour = world.colour_at(ray);

            canvas.set(x, y, colour)
        }
    }

    canvas
}
