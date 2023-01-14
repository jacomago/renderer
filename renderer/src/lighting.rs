use colors::prelude::Color;
use geometry::prelude::Vector3D;

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
}
