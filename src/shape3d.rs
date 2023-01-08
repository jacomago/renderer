use crate::{
    color::Color,
    prelude::Coloring,
    shape2d::Colored,
    vectors::{Position2D, Ray, Vector3D},
};

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

pub trait Shape3d {
    fn intersection(&self, ray: &Ray) -> Option<Vector3D<f32>>;
    fn color(&self, position: &Vector3D<f32>) -> Option<Color>;
}
impl Shape3d for Sphere {
    // (v- p)(v-p) = r**2
    // (ray_v - p)(ray_v - p) = r**2
    fn intersection(&self, ray: &Ray) -> Option<Vector3D<f32>> {
        todo!()
    }

    fn color(&self, position: &Vector3D<f32>) -> Option<Color> {
        todo!()
    }
}
