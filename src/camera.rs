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

    pub fn ray(&self, position: &Position2D<usize>) -> Ray {
        Ray(Vector3D::default())
    }

    pub fn position(&self) -> Vector3D<f32> {
        self.position
    }
}
