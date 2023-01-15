use colors::prelude::Color;
use geometry::prelude::{Ray, Vector3D};

pub struct Light {
    position: Vector3D<f32>,
    color: Color<f32>,
    intensity: Color<f32>,
}

impl Light {
    pub fn new(position: Vector3D<f32>, color: Color<f32>, intensity: Color<f32>) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
    pub fn color(&self) -> &Color<f32> {
        &self.color
    }
    pub fn position(&self) -> &Vector3D<f32> {
        &self.position
    }
    pub fn ray(&self, position: &Vector3D<f32>) -> Ray {
        Ray::new(self.position, self.position - *position)
    }
}
