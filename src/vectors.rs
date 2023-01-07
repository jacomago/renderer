use std::ops::{Add, Mul, Sub};

pub struct Ray(pub Vector3D<f32>);

pub struct Position2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Position2D { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default)]
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

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn length_squared(&self) -> T {
        self.dot(&self)
    }
}
impl<T: Mul<Output = T> + Add<Output = T> + Copy + Sub<Output = T>> Vector3D<T> {
    pub fn distance_squared(self, rhs: Self) -> T {
        (self - rhs).length_squared()
    }
}

impl<T: Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Vector3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}