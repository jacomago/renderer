use std::ops::Mul;

use crate::monotone::Monotone;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color<T> {
    red: T,
    blue: T,
    green: T,
    alpha: T,
}

impl<T> Color<T> {
    pub fn new(red: T, blue: T, green: T, alpha: T) -> Self {
        Self {
            red,
            blue,
            green,
            alpha,
        }
    }

    pub fn red(&self) -> &T {
        &self.red
    }

    pub fn blue(&self) -> &T {
        &self.blue
    }

    pub fn green(&self) -> &T {
        &self.green
    }

    pub fn alpha(&self) -> &T {
        &self.alpha
    }
}

impl Color<f32> {
    pub fn monotone(&self) -> Monotone {
        if self.red + self.green + self.blue > 1.5 {
            return Monotone::Black;
        }
        Monotone::White
    }
    pub const BLACK: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 1.0,
    };
    pub const WHITE: Self = Self {
        red: 1.0,
        blue: 1.0,
        green: 1.0,
        alpha: 1.0,
    };
    pub const RED: Self = Self {
        red: 1.0,
        blue: 0.0,
        green: 0.0,
        alpha: 1.0,
    };
    pub const GREEN: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 1.0,
        alpha: 1.0,
    };
    pub const BLUE: Self = Self {
        red: 0.0,
        blue: 1.0,
        green: 0.0,
        alpha: 1.0,
    };
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
    pub fn rgb_bytes(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}
