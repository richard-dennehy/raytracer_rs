mod render;
pub use render::{render, Samples};

mod camera;
pub use camera::Camera;

mod canvas;
pub use canvas::Canvas;

#[cfg(test)]
mod tests {
    use super::*;

    mod camera_tests;
    mod canvas_tests;
    mod render_tests;
}
