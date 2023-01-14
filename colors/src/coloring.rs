use crate::{color::Color, gradient::Gradient};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Coloring<T> {
    Fill(Color<T>),
    Gradient(Gradient<T>),
}

impl<T: Default> Default for Coloring<T> {
    fn default() -> Self {
        Coloring::Fill(Color::default())
    }
}

impl Coloring<f32> {
    pub fn color(&self, percentage: f32) -> Color<f32> {
        match self {
            Coloring::Fill(c) => *c,
            Coloring::Gradient(g) => g.color(percentage),
        }
    }

    pub fn gradient(colors: (Color<f32>, Color<f32>)) -> Self {
        Self::Gradient(Gradient::new(colors))
    }
}
