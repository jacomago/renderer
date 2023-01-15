use crate::color::Color;

/// Gradient represents a selection of multiple colors
/// and a method to linearly interpolate between them
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Gradient<T> {
    colors: (Color<T>, Color<T>),
}

fn linearly_iterpolate(x: f32, y: f32, percent: f32) -> f32 {
    let value: f32 = x + percent * (y - x);
    if value < 0.0 {
        return 0.0;
    }
    if value > 1.0 {
        return 1.0;
    }
    value
}

impl<T> Gradient<T> {
    /// Constructor for a gradient
    pub fn new(colors: (Color<T>, Color<T>)) -> Self {
        Self { colors }
    }
}

impl Gradient<f32> {
    /// Get the color givena percentage of distance along the interpolation
    pub fn color(&self, percentage: f32) -> Color<f32> {
        Color::new(
            linearly_iterpolate(*self.colors.0.red(), *self.colors.1.red(), percentage),
            linearly_iterpolate(*self.colors.0.blue(), *self.colors.1.blue(), percentage),
            linearly_iterpolate(*self.colors.0.green(), *self.colors.1.green(), percentage),
            linearly_iterpolate(*self.colors.0.alpha(), *self.colors.1.alpha(), percentage),
        )
    }
}
