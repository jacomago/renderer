use std::{fmt::Debug, vec};

use crate::{
    normal::Normal,
    prelude::Ray,
    ray_intersections::RayIntersections,
    shape::{Contains, HorizontalPercentage},
    vectors::Vector3D,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Sphere {
    position: Vector3D<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(position: Vector3D<f32>, radius: f32) -> Self {
        Self { position, radius }
    }
}

impl RayIntersections for Sphere {
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

impl Contains<Vector3D<f32>> for Sphere {
    fn contains(&self, position: &Vector3D<f32>) -> bool {
        self.position.dot(position) < self.radius.powi(2)
    }
}

impl HorizontalPercentage<Vector3D<f32>> for Sphere {
    fn horizontal_percentage(&self, position: &Vector3D<f32>) -> f32 {
        (self.position.x() - self.radius - position.x()).abs() / (2.0 * self.radius)
    }
}

impl Normal<f32> for Sphere {
    fn normal(&self, p: &Vector3D<f32>) -> Vector3D<f32> {
        *p - self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_origin() {
        let sphere = Sphere::new(Vector3D::default(), 1.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(
            vec![Vector3D::new(0.0, 0.0, 1.0), Vector3D::new(0.0, 0.0, -1.0)],
            intersections
        );
    }
    #[test]
    fn test_intersection_one() {
        let sphere = Sphere::new(Vector3D::new(1.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(vec![Vector3D::new(0.0, 0.0, 0.0)], intersections);
    }
    #[test]
    fn test_intersection_none() {
        let sphere = Sphere::new(Vector3D::new(1.5, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(Vec::<Vector3D<f32>>::new(), intersections);
    }
    #[test]
    fn test_small_radius() {
        let sphere = Sphere::new(Vector3D::new(1.0, 0.0, 0.0), 0.5);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(1.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(Vec::<Vector3D<f32>>::new(), intersections);
    }
    #[test]
    fn test_large_radius() {
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 2.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(
            vec![Vector3D::new(0.0, 0.0, 2.0), Vector3D::new(0.0, 0.0, -2.0)],
            intersections
        );
    }
    #[test]
    fn test_shift_x() {
        let sphere = Sphere::new(Vector3D::new(2.0, 0.0, 0.0), 2.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(vec![Vector3D::new(0.0, 0.0, 0.0)], intersections);
    }
    #[test]
    fn test_shift_y() {
        let sphere = Sphere::new(Vector3D::new(0.0, 2.0, 0.0), 2.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(vec![Vector3D::new(0.0, 0.0, 0.0)], intersections);
    }
    #[test]
    fn test_shift_z() {
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 2.0), 2.0);
        let ray = Ray::new(Vector3D::default(), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = sphere.intersection(&ray);

        assert_eq!(
            vec![Vector3D::new(0.0, 0.0, 4.0), Vector3D::new(0.0, 0.0, 0.0)],
            intersections
        );
    }
}
