use std::iter::Sum;
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

    pub const fn greyscale(c: f64) -> Self {
        Colour(c, c, c)
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

    /// the proportion (0.0 to 1.0) that red contributes to this colour
    pub fn red_factor(&self) -> f64 {
        self.red() / self.intensity()
    }

    /// the proportion (0.0 to 1.0) that green contributes to this colour
    pub fn green_factor(&self) -> f64 {
        self.green() / self.intensity()
    }

    /// the proportion (0.0 to 1.0) that blue contributes to this colour
    pub fn blue_factor(&self) -> f64 {
        self.blue() / self.intensity()
    }

    /// scales the RGB components such that R+G+B ~= 1.0 - intended to calculate light passing through
    /// coloured transparent materials
    ///
    /// Note that `Colour::BLACK.normalised() == Colour::BLACK`
    pub fn normalised(&self) -> Self {
        let magnitude = self.intensity();

        if magnitude == 0.0 {
            *self
        } else {
            Colour::new(
                self.red() / magnitude,
                self.green() / magnitude,
                self.blue() / magnitude,
            )
        }
    }

    pub fn intensity(&self) -> f64 {
        self.red() + self.blue() + self.green()
    }

    pub fn average(self, other: Self) -> Self {
        (self + other) * 0.5
    }

    /// returns `true` when `self` is imperceptibly different to `other` i.e. when the colours are
    /// interchangeable
    ///
    /// assumes RGB colour range of 0-255
    pub fn is_similar_to(&self, other: &Self) -> bool {
        (self.red() * 255.0) as usize == (other.red() * 255.0) as usize
            && (self.green() * 255.0) as usize == (other.green() * 255.0) as usize
            && (self.blue() * 255.0) as usize == (other.blue() * 255.0) as usize
    }
}

impl Default for Colour {
    fn default() -> Self {
        Colour::BLACK
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

impl Add<&Colour> for &Colour {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Self::Output {
        *self + *rhs
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

impl Mul<f64> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        *self * rhs
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

impl Sum for Colour {
    fn sum<I: Iterator<Item = Colour>>(iter: I) -> Self {
        iter.fold(Colour::BLACK, |acc, next| acc + next)
    }
}

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use crate::Colour;
    use float_cmp::{ApproxEq, F64Margin};

    impl ApproxEq for Colour {
        type Margin = F64Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();

            self.0.approx_eq(other.0, margin)
                && self.1.approx_eq(other.1, margin)
                && self.2.approx_eq(other.2, margin)
        }
    }
}
