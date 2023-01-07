use crate::{
    dimensions::Dimensions,
    vectors::{Position2D, Ray, Vector3D},
};

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
    pub screen: Screen,
}

impl Camera {
    pub fn new(position: Vector3D<f32>, screen: Screen) -> Self {
        Self { position, screen }
    }

    pub fn focal_distance(&self) -> f32 {
        (self.position - self.screen.position)
            .length_squared()
            .sqrt()
    }

    pub fn ray(&self, position: &Position2D<usize>) -> Ray {
        
        Ray(Vector3D::default())
    }

    pub fn position(&self) -> Vector3D<f32> {
        self.position
    }
}
