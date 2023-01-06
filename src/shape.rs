use crate::{color::Color, image::Image, position::Position};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

pub trait Colored {
    fn color(&self, position: &Position<usize>) -> Option<Color>;
}

pub struct Circle {
    radius: f32,
    position: Position<f32>,
    color: Color,
}

impl Circle {
    pub fn new(radius: f32, position: Position<f32>, color: Color) -> Self {
        Self {
            radius,
            position,
            color,
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
            return Some(self.color);
        }
        None
    }
}
