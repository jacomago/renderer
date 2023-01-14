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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub const RED: Self = Self {
        red: 255,
        blue: 0,
        green: 0,
        alpha: 255,
    };

    pub const YELLOW: Self = Self {
        red: 255,
        blue: 255,
        green: 0,
        alpha: 255,
    };

    pub fn rgb_bytes(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Gradient {
    colors: (Color, Color),
}

fn linearly_iterpolate(x: u8, y: u8, percent: f32) -> u8 {
    let value: f32 = x as f32 + percent * (y as f32 - x as f32);
    if value < u8::MIN.into() {
        return u8::MIN;
    }
    if value > u8::MAX.into() {
        return u8::MAX;
    }
    value as u8
}

impl Gradient {
    fn color(&self, percentage: f32) -> Color {
        Color {
            red: linearly_iterpolate(self.colors.0.red, self.colors.1.red, percentage),
            blue: linearly_iterpolate(self.colors.0.blue, self.colors.1.blue, percentage),
            green: linearly_iterpolate(self.colors.0.green, self.colors.1.green, percentage),
            alpha: linearly_iterpolate(self.colors.0.alpha, self.colors.1.alpha, percentage),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Coloring {
    Fill(Color),
    Gradient(Gradient),
}
impl Default for Coloring {
    fn default() -> Self {
        Coloring::Fill(Color::default())
    }
}
impl Coloring {
    pub fn color(&self, percentage: f32) -> Color {
        match self {
            Coloring::Fill(c) => *c,
            Coloring::Gradient(g) => g.color(percentage),
        }
    }

    pub fn gradient(colors: (Color, Color)) -> Self {
        Self::Gradient(Gradient { colors })
    }
}
