use colors::prelude::Color;
use geometry::prelude::{Normal, Ray, Vector3D};

use crate::colored::Colored;

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

    pub fn ray(&self, position: &Vector3D<f32>) -> Ray {
        Ray::new(self.position, self.position - *position)
    }

    pub fn angle(&self, shape: &dyn Normal<f32>, position: &Vector3D<f32>) -> f32 {
        let r = self.position - *position;
        let normalized_r = r.normalize();
        normalized_r.dot(&shape.normal(position))
    }

    pub fn surface_color(
        &self,
        shape: &dyn Colored<Vector3D<f32>>,
        position: &Vector3D<f32>,
        surface_coeff: f32,
    ) -> Color<f32> {
        let shape_color = shape.color(position).unwrap_or_default();
        self.color * shape_color * surface_coeff
    }
}
