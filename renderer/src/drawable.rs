use geometry::prelude::{Circle, Contains, HorizontalPercentage, Position2D};
use image_lib::prelude::{Coloring, Image};

use crate::colored::Colored;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

impl Drawable for ColoredCircle {
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

pub struct ColoredCircle {
    circle: Circle,
    coloring: Coloring,
}

impl ColoredCircle {
    pub fn new(circle: Circle, coloring: Coloring) -> Self {
        Self { circle, coloring }
    }
}

impl Contains<Position2D<usize>> for ColoredCircle {
    fn contains(&self, position: &Position2D<usize>) -> bool {
        self.circle.contains(position)
    }
}
impl HorizontalPercentage<Position2D<usize>> for ColoredCircle {
    fn horizontal_percentage(&self, position: &Position2D<usize>) -> f32 {
        self.circle.horizontal_percentage(position)
    }
}
impl Colored<Position2D<usize>> for ColoredCircle {
    fn coloring(&self) -> &Coloring {
        &self.coloring
    }
}
