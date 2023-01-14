use colors::prelude::{Color, Coloring};
use geometry::prelude::{Contains, HorizontalPercentage};

pub trait Colored<T>: Contains<T> + HorizontalPercentage<T> {
    fn coloring(&self) -> &Coloring<f32>;
    fn color(&self, position: &T) -> Option<Color<f32>> {
        if self.contains(position) {
            return Some(self.coloring().color(self.horizontal_percentage(position)));
        }
        None
    }
}
