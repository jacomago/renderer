
pub trait HorizontalPercentage<T> {
    fn horizontal_percentage(&self, position: &T) -> f32;
}

pub trait Contains<T> {
    fn contains(&self, position: &T) -> bool;
}
