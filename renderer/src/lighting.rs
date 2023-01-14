use geometry::prelude::Vector3D;
use image_lib::prelude::Color;

pub struct Light {
    position: Vector3D<f32>,
    color: Color,
    intensity: Color,
}

impl Light {
    pub fn new(position: Vector3D<f32>, color: Color, intensity: Color) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}
