use crate::position::Vector3D;

pub struct Sphere {
    position: Vector3D<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(position: Vector3D<f32>, radius: f32) -> Self {
        Self { position, radius }
    }
}
