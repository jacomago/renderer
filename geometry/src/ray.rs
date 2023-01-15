use std::f32::NAN;

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

    pub fn t(&self, pos: Vector3D<f32>) -> f32 {
        let diff = pos - self.starting_point;
        if diff.x().is_normal() && self.direction.x().is_normal() {
            return diff.x() / self.direction.x();
        }
        if diff.y().is_normal() && self.direction.y().is_normal() {
            return diff.y() / self.direction.y();
        }
        if diff.z().is_normal() && self.direction.z().is_normal() {
            return diff.z() / self.direction.z();
        }
        NAN
    }
}
