use crate::{
    color::{Color, Coloring},
    image::Image,
    position::Position,
};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

pub trait Colored {
    fn color(&self, position: &Position<usize>) -> Option<Color>;
}

pub struct Circle {
    radius: f32,
    position: Position<f32>,
    coloring: Coloring,
}

impl Circle {
    pub fn new(radius: f32, position: Position<f32>, coloring: Coloring) -> Self {
        Self {
            radius,
            position,
            coloring,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        image.positions().iter().for_each(|p| {
            if let Some(color) = self.color(p) {
                if let Some(pixel) = image.pixel_mut(p) {
                    pixel.color = color;
                }
            }
        })
    }
}

impl Colored for Circle {
    fn color(&self, position: &Position<usize>) -> Option<Color> {
        if (self.position.x - position.x as f32).powi(2)
            + (self.position.y - position.y as f32).powi(2)
            < self.radius.powi(2)
        {
            return Some(self.coloring.color(self.horizontal_percentage(position)));
        }
        None
    }
}
trait HorizontalPercentage {
    fn horizontal_percentage(&self, position: &Position<usize>) -> f32;
}

impl HorizontalPercentage for Circle {
    fn horizontal_percentage(&self, position: &Position<usize>) -> f32 {
        (self.position.x - self.radius - position.x as f32).abs() / (2.0 * self.radius)
    }
}
