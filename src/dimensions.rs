use crate::vectors::Position2D;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions<T: Copy> {
    width: T,
    height: T,
}

impl<T: Copy> Dimensions<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Copy + Into<usize> + From<usize>> Dimensions<T> {
    pub fn w(&self) -> T {
        self.width
    }
    pub fn h(&self) -> T {
        self.height
    }
    pub fn positions(&self) -> Vec<Position2D<T>> {
        (0..self.h().into())
            .flat_map(|col| {
                (0..self.w().into()).map(move |row| Position2D::new(row.into(), col.into()))
            })
            .collect()
    }
}

impl From<Dimensions<usize>> for Dimensions<i32> {
    fn from(d: Dimensions<usize>) -> Self {
        Dimensions::new(d.width as i32, d.height as i32)
    }
}

impl From<Dimensions<i32>> for Dimensions<usize> {
    fn from(d: Dimensions<i32>) -> Self {
        Dimensions::new(d.width as usize, d.height as usize)
    }
}
