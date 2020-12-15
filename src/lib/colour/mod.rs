use std::ops::{Add, Mul, Sub};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Colour(f64, f64, f64);

#[allow(dead_code)]
impl Colour {
    pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
    pub const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
    pub const RED: Colour = Colour::new(1.0, 0.0, 0.0);
    pub const GREEN: Colour = Colour::new(0.0, 1.0, 0.0);
    pub const BLUE: Colour = Colour::new(0.0, 0.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Colour(r, g, b)
    }

    pub const fn red(&self) -> f64 {
        self.0
    }

    pub const fn green(&self) -> f64 {
        self.1
    }

    pub const fn blue(&self) -> f64 {
        self.2
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
}

impl Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Self::Output {
        Colour(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue() - rhs.blue(),
        )
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour(
            self.red() * rhs.red(),
            self.green() * rhs.green(),
            self.blue() * rhs.blue(),
        )
    }
}
