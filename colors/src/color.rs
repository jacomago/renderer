use std::ops::Mul;

use crate::monotone::Monotone;

/// Color represents a color with red, green, blue and alpha values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color<T> {
    red: T,
    blue: T,
    green: T,
    alpha: T,
}

impl<T> Color<T> {
    /// A constructor for Color
    pub fn new(red: T, blue: T, green: T, alpha: T) -> Self {
        Self {
            red,
            blue,
            green,
            alpha,
        }
    }

    /// Red part of color
    pub fn red(&self) -> &T {
        &self.red
    }

    /// Blue part of color
    pub fn blue(&self) -> &T {
        &self.blue
    }

    /// Green part of color
    pub fn green(&self) -> &T {
        &self.green
    }

    /// Alpha part of color
    pub fn alpha(&self) -> &T {
        &self.alpha
    }
}

impl Color<f32> {
    /// Convert color to black or white
    pub fn monotone(&self) -> Monotone {
        if self.red + self.green + self.blue > 1.5 {
            return Monotone::Black;
        }
        Monotone::White
    }
    /// Constant black color
    pub const BLACK: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 1.0,
    };
    /// Constant white color
    pub const WHITE: Self = Self {
        red: 1.0,
        blue: 1.0,
        green: 1.0,
        alpha: 1.0,
    };

    /// Constant red color
    pub const RED: Self = Self {
        red: 1.0,
        blue: 0.0,
        green: 0.0,
        alpha: 1.0,
    };
    /// Constant green color
    pub const GREEN: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 1.0,
        alpha: 1.0,
    };

    /// Constant blue color
    pub const BLUE: Self = Self {
        red: 0.0,
        blue: 1.0,
        green: 0.0,
        alpha: 1.0,
    };
    /// Constant yellow color
    pub const YELLOW: Self = Self {
        red: 1.0,
        blue: 0.0,
        green: 1.0,
        alpha: 1.0,
    };
}

impl<T: Mul<Output = T>> Mul for Color<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            blue: self.blue * rhs.blue,
            green: self.green * rhs.green,
            alpha: self.alpha * rhs.alpha,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            red: self.red * rhs,
            blue: self.blue * rhs,
            green: self.green * rhs,
            alpha: self.alpha * rhs,
        }
    }
}

fn float_to_u8(f: f32) -> u8 {
    (f * 255.0) as u8
}

impl From<Color<f32>> for Color<u8> {
    fn from(value: Color<f32>) -> Self {
        Color {
            red: float_to_u8(value.red),
            blue: float_to_u8(value.blue),
            green: float_to_u8(value.green),
            alpha: float_to_u8(value.alpha),
        }
    }
}

impl Color<u8> {
    /// Convert color to vector of bytes
    /// Useful for writing into an image format
    pub fn rgb_bytes(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}
