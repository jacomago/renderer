use crate::{
    shape::{Contains, HorizontalPercentage},
    vectors::Position2D,
};

pub struct Circle {
    radius: f32,
    position: Position2D<f32>,
}

impl Circle {
    pub fn new(radius: f32, position: Position2D<f32>) -> Self {
        Self { radius, position }
    }
}

impl Contains<Position2D<usize>> for Circle {
    fn contains(&self, position: &Position2D<usize>) -> bool {
        (self.position.x - position.x as f32).powi(2)
            + (self.position.y - position.y as f32).powi(2)
            < self.radius.powi(2)
    }
}
impl HorizontalPercentage<Position2D<usize>> for Circle {
    fn horizontal_percentage(&self, position: &Position2D<usize>) -> f32 {
        (self.position.x - self.radius - position.x as f32).abs() / (2.0 * self.radius)
    }
}
