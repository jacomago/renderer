use crate::{color::Color, gradient::Gradient};

/// How to color a shape or object
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Coloring<T> {
    /// Fill is one sold block of color
    Fill(Color<T>),
    /// Gradient is a change of colors across a direction
    Gradient(Gradient<T>),
}

impl<T: Default> Default for Coloring<T> {
    fn default() -> Self {
        Coloring::Fill(Color::default())
    }
}

impl Coloring<f32> {
    /// Get the color from a coloring
    pub fn color(&self, percentage: f32) -> Color<f32> {
        match self {
            Coloring::Fill(c) => *c,
            Coloring::Gradient(g) => g.color(percentage),
        }
    }

    /// Create a gradient coloring based on a pair of colors
    pub fn gradient(colors: (Color<f32>, Color<f32>)) -> Self {
        Self::Gradient(Gradient::new(colors))
    }
}
