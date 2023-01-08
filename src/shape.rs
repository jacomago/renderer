use crate::color::Color;

pub trait HorizontalPercentage<T> {
    fn horizontal_percentage(&self, position: &T) -> f32;
}

pub trait Colored<T> {
    fn color(&self, position: &T) -> Option<Color>;
}

