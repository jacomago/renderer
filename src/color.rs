use std::fmt::Display;

pub enum Monotone {
    Black,
    White,
}

impl Display for Monotone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Monotone::Black => write!(f, "x"),
            Monotone::White => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    blue: u8,
    green: u8,
    alpha: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: Default::default(),
            blue: Default::default(),
            green: Default::default(),
            alpha: 255,
        }
    }
}

impl Color {
    pub fn monotone(&self) -> Monotone {
        if self.red as u16 + self.green as u16 + self.blue as u16 > 128 * 3 {
            return Monotone::Black;
        }
        Monotone::White
    }
    pub const BLACK: Self = Self {
        red: 0,
        blue: 0,
        green: 0,
        alpha: 255,
    };
    pub const WHITE: Self = Self {
        red: 255,
        blue: 255,
        green: 255,
        alpha: 255,
    };

    pub fn rgb_bytes(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}
