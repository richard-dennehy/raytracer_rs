use crate::core::Ray;
use crate::core::{Point3D, Transform, Vector};
use std::num::NonZeroU16;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Camera {
    width: NonZeroU16,
    height: NonZeroU16,
    transform: Transform,
    pixel_size: f64,
    half_canvas_width: f64,
    half_canvas_height: f64,
}

impl Camera {
    pub fn new(
        width: NonZeroU16,
        height: NonZeroU16,
        fov_radians: f64,
        transform: Transform,
    ) -> Self {
        let w = width.get() as f64;
        let h = height.get() as f64;

        let half_view = (fov_radians / 2.0).tan();
        let aspect_ratio = w / h;

        let (half_canvas_width, half_canvas_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };

        let pixel_size = (half_canvas_width * 2.0) / w;

        Camera {
            width,
            height,
            transform,
            pixel_size,
            half_canvas_width,
            half_canvas_height,
        }
    }

    pub fn ray_at(&self, x: u16, y: u16, x_offset: f64, y_offset: f64) -> Ray {
        let x_offset = (x as f64 + x_offset) * self.pixel_size;
        let y_offset = (y as f64 + y_offset) * self.pixel_size;

        let world_x = self.half_canvas_width - x_offset;
        let world_y = self.half_canvas_height - y_offset;

        let inverse = self.transform.inverse();

        let (x, y, z, _) = &inverse * Point3D::new(world_x, world_y, -1.0);
        let pixel = Point3D::new(x, y, z);

        let (x, y, z, _) = inverse * Point3D::new(0.0, 0.0, 0.0);
        let origin = Point3D::new(x, y, z);
        let direction = (pixel - origin).normalised();

        Ray::new(origin, direction)
    }

    pub fn width(&self) -> NonZeroU16 {
        self.width
    }

    pub fn height(&self) -> NonZeroU16 {
        self.height
    }
}
