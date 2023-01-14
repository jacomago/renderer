use crate::prelude::Vector3D;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub starting_point: Vector3D<f32>,
    pub direction: Vector3D<f32>,
}

impl Ray {
    pub fn new(starting_point: Vector3D<f32>, direction: Vector3D<f32>) -> Self {
        Self {
            starting_point,
            direction,
        }
    }

    pub fn eval(&self, t: f32) -> Vector3D<f32> {
        self.starting_point + self.direction * t
    }
}
