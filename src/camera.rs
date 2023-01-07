use crate::{image::Dimensions, position::Vector3D};

pub struct Direction {
    up: Vector3D<f32>,
    right: Vector3D<f32>,
}

impl Direction {
    pub fn new(up: Vector3D<f32>, right: Vector3D<f32>) -> Self {
        Self { up, right }
    }
}

pub struct Screen {
    size: Dimensions,
    position: Vector3D<f32>,
}

impl Screen {
    fn new(size: Dimensions, position: Vector3D<f32>) -> Self {
        Self { size, position }
    }
}

pub struct Camera {
    position: Vector3D<f32>,
    direction: Direction,
    screen: Screen,
}

impl Camera {
    pub fn new(position: Vector3D<f32>, direction: Direction, screen: Screen) -> Self {
        Self {
            position,
            direction,
            screen,
        }
    }
}
