use crate::{prelude::Coloring, vectors::Vector3D};

pub struct Sphere {
    position: Vector3D<f32>,
    radius: f32,
    coloring: Coloring,
}

impl Sphere {
    pub fn new(position: Vector3D<f32>, radius: f32, coloring: Coloring) -> Self {
        Self {
            position,
            radius,
            coloring,
        }
    }
}

pub trait Shape3d {}
impl Shape3d for Sphere {}
