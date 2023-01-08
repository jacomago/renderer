use std::vec;

use crate::{
    prelude::Coloring,
    shape::{Colored, Contains, HorizontalPercentage},
    vectors::{Ray, Vector3D},
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

pub trait Shape3d: Colored<Vector3D<f32>> {
    fn intersection(&self, ray: &Ray) -> Vec<Vector3D<f32>>;
}
impl Shape3d for Sphere {
    // (v- p)(v-p) = r**2
    // (ray_v - p)(ray_v - p) = r**2
    // (o + dt - p)(o+dt -p) = r**2
    // let c = o-p
    // d**2 * t**2 + 2 c.d t + (c**2 - r**2) = 0
    fn intersection(&self, ray: &Ray) -> Vec<Vector3D<f32>> {
        let new_centre = ray.starting_point - self.position;
        let d_squared = ray.direction.dot(&ray.direction);
        let c_dot_d = ray.direction.dot(&new_centre);
        let c_squared = new_centre.dot(&new_centre);

        // quadratic equation = -b + sqrt(b**2 -4ac) / 2a
        let a = d_squared;
        let b = 2.0 * c_dot_d;
        let c = c_squared - self.radius.powi(2);
        let determinant = b.powi(2) - 4.0 * a * c;
        if determinant < 0.0 {
            return vec![];
        }
        let det_sqrt = determinant.sqrt();
        let first_t = (-b + det_sqrt) / (2.0 * a);
        let second_t = (-b - det_sqrt) / (2.0 * a);
        if (first_t - second_t).abs() < f32::EPSILON {
            vec![ray.eval(first_t)]
        } else {
            vec![ray.eval(first_t), ray.eval(second_t)]
        }
    }
}

impl HorizontalPercentage<Vector3D<f32>> for Sphere {
    fn horizontal_percentage(&self, position: &Vector3D<f32>) -> f32 {
        (self.position.x - self.radius - position.x as f32).abs() / (2.0 * self.radius)
    }
}
impl Contains<Vector3D<f32>> for Sphere {
    fn contains(&self, position: &Vector3D<f32>) -> bool {
        (self.position.x - position.x as f32).powi(2)
            + (self.position.y - position.y as f32).powi(2)
            + (self.position.z - position.z as f32).powi(2)
            < self.radius.powi(2)
    }
}

impl Colored<Vector3D<f32>> for Sphere {
    fn coloring(&self) -> &Coloring {
        &self.coloring
    }
}
