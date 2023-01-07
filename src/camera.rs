use crate::{dimensions::Dimensions, vectors::Vector3D};

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
    pub size: Dimensions<i32>,
    position: Vector3D<f32>,
}

impl Screen {
    pub fn new(size: Dimensions<i32>, position: Vector3D<f32>) -> Self {
        Self { size, position }
    }
}

pub struct Camera {
    position: Vector3D<f32>,
    direction: Direction,
    pub screen: Screen,
}

impl Camera {
    pub fn new(position: Vector3D<f32>, direction: Direction, screen: Screen) -> Self {
        Self {
            position,
            direction,
            screen,
        }
    }

    pub fn focal_distance(&self) -> f32 {
        (self.position - self.screen.position)
            .distance_squared()
            .sqrt()
    }
}
