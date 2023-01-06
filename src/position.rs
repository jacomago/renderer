
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Position { x, y }
    }
}