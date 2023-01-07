#[derive(Debug, Clone, Copy)]
pub struct Dimensions<T: Copy> {
    width: T,
    height: T,
}

impl<T: Copy> Dimensions<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
    pub fn w(&self) -> T {
        self.width
    }
    pub fn h(&self) -> T {
        self.height
    }
}

impl From<Dimensions<usize>> for Dimensions<i32> {
    fn from(d: Dimensions<usize>) -> Self {
        Dimensions::new(d.width as i32, d.h() as i32)
    }
}

impl From<Dimensions<i32>> for Dimensions<usize> {
    fn from(d: Dimensions<i32>) -> Self {
        Dimensions::new(d.width as usize, d.h() as usize)
    }
}
