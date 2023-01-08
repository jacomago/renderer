use crate::{
    dimensions::Dimensions,
    vectors::{Position2D, Ray, Vector3D},
};

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
    pub size: Dimensions<usize>,
}

impl Screen {
    pub fn new(size: Dimensions<usize>) -> Self {
        Self { size }
    }
    pub fn positions(&self) -> Vec<Position2D<usize>> {
        self.size.positions()
    }
}

pub struct Camera {
    position: Vector3D<f32>,
    direction: Direction,
    pub screen: Screen,
    focal_distance: f32,
}

impl Camera {
    pub fn new(
        position: Vector3D<f32>,
        direction: Direction,
        screen: Screen,
        focal_distance: f32,
    ) -> Self {
        Self {
            position,
            direction,
            screen,
            focal_distance,
        }
    }

    pub fn centre(&self) -> Vector3D<f32> {
        self.position + (self.direction.up + self.direction.right) * self.focal_distance
    }

    pub fn ray(&self, position: &Position2D<usize>) -> Ray {
        let screen_position = self.centre()
            + self.direction.right * position.x as f32
            + self.direction.up * position.y as f32;
        let d = screen_position - self.position;
        Ray::new(self.position, d)
    }

    pub fn position(&self) -> Vector3D<f32> {
        self.position
    }
}
