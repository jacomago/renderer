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
