use crate::Colour;
use std::num::NonZeroU16;

#[cfg(test)]
mod tests;

pub struct Canvas(Vec<Vec<Colour>>);

impl Canvas {
    /// creates a `Canvas` of `width` by `height` dimensions
    /// returns `None` if the dimensions are too great, to prevent allocating ridiculous amounts of memory
    /// (specifically: width greater than 7680 or height greater than 4320 (i.e. ~16K))
    /// (note: 65535 by 65535 would create a ~12GB data structure)
    pub fn new(width: NonZeroU16, height: NonZeroU16) -> Option<Self> {
        let height = height.get();
        let width = width.get();

        if width > (1920 * 4) || height > (1080 * 4) {
            return None;
        }

        let mut underlying = Vec::with_capacity(height as _);

        for _ in 0..underlying.capacity() {
            let mut row = Vec::with_capacity(width as _);

            for _ in 0..row.capacity() {
                row.push(Colour::BLACK);
            }

            underlying.push(row);
        }

        Some(Canvas(underlying))
    }

    pub fn width(&self) -> usize {
        self.0
            .first()
            .expect("underlying Vec cannot be empty")
            .len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// # Panics
    /// Panics if `x` or `y` are out of bounds (0..width-1 and 0..height-1)
    pub fn get(&self, x: u16, y: u16) -> Colour {
        let x = x as usize;
        let y = y as usize;

        self.0[y][x]
    }

    /// # Panics
    /// Panics if `x` or `y` are out of bounds (0..width-1 and 0..height-1)
    pub fn set(&mut self, x: u16, y: u16, colour: Colour) {
        let x = x as usize;
        let y = y as usize;

        self.0[y][x] = colour
    }
}
