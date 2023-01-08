use crate::color::{Color, Coloring};

pub trait HorizontalPercentage<T> {
    fn horizontal_percentage(&self, position: &T) -> f32;
}
pub trait Contains<T> {
    fn contains(&self, position: &T) -> bool;
}
pub trait Colored<T>: Contains<T> + HorizontalPercentage<T> {
    fn coloring(&self) -> &Coloring;
    fn color(&self, position: &T) -> Option<Color> {
        if self.contains(position) {
            return Some(self.coloring().color(self.horizontal_percentage(position)));
        }
        None
    }
}
