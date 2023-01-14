use geometry::prelude::{Contains, HorizontalPercentage, RayIntersections, Sphere, Vector3D};
use image_lib::prelude::Coloring;

use crate::colored::Colored;

#[derive(Clone, Copy)]
pub struct SceneObject {
    sphere: Sphere,
    coloring: Coloring,
}

impl SceneObject {
    pub fn new(sphere: Sphere, coloring: Coloring) -> Self {
        Self { sphere, coloring }
    }
}

impl RayIntersections for SceneObject {
    fn intersection(&self, ray: &geometry::prelude::Ray) -> Vec<geometry::prelude::Vector3D<f32>> {
        self.sphere.intersection(ray)
    }
}

impl Contains<Vector3D<f32>> for SceneObject {
    fn contains(&self, position: &Vector3D<f32>) -> bool {
        self.sphere.contains(position)
    }
}
impl HorizontalPercentage<Vector3D<f32>> for SceneObject {
    fn horizontal_percentage(&self, position: &Vector3D<f32>) -> f32 {
        self.sphere.horizontal_percentage(position)
    }
}
impl Colored<Vector3D<f32>> for SceneObject {
    fn coloring(&self) -> &Coloring {
        &self.coloring
    }
}
