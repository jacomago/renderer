use std::ops::{Add, Mul};

pub struct Position2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Position2D { x, y }
    }
}

pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Vector3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Vector3D<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

