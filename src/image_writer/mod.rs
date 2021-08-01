use crate::renderer::Canvas;
use image::{ImageBuffer, Rgb, RgbImage};

pub fn write(canvas: Canvas) -> RgbImage {
    ImageBuffer::from_fn(canvas.width() as _, canvas.height() as _, |x, y| {
        let colour = canvas.get(x as _, y as _);

        Rgb([
            clamp(colour.red()),
            clamp(colour.green()),
            clamp(colour.blue()),
        ])
    })
}

fn clamp(c: f64) -> u8 {
    if c <= 0.0 {
        0
    } else if c >= 1.0 {
        u8::MAX
    } else {
        (255.0 * c).round() as _
    }
}
