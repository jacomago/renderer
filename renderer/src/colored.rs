use geometry::prelude::{Contains, HorizontalPercentage};
use image_lib::prelude::{Color, Coloring};

pub trait Colored<T>: Contains<T> + HorizontalPercentage<T> {
    fn coloring(&self) -> &Coloring;
    fn color(&self, position: &T) -> Option<Color> {
        if self.contains(position) {
            return Some(self.coloring().color(self.horizontal_percentage(position)));
        }
        None
    }
}
