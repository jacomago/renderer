use std::cmp::Ordering;

use crate::{
    shape3d::RayIntersections,
    vectors::{Ray, Vector3D},
};

fn distance_squared_cmp(i: &Vector3D<f32>, j: &Vector3D<f32>, origin: Vector3D<f32>) -> Ordering {
    i.distance_squared(origin)
        .total_cmp(&j.distance_squared(origin))
}

pub fn closest_intersection<T: RayIntersections + Copy>(
    objects: &Vec<T>,
    ray: &Ray,
    origin: Vector3D<f32>,
) -> Option<(T, Vector3D<f32>)> {
    objects
        .iter()
        .map(|object| (object, object.intersection(ray)))
        .filter(|(_, i)| !i.is_empty())
        .map(|(o, i)| {
            (
                *o,
                *i.iter()
                    .min_by(|i, j| distance_squared_cmp(i, j, origin))
                    .unwrap(),
            )
        })
        .min_by(|(_, i), (_, j)| distance_squared_cmp(i, j, origin))
}

#[cfg(test)]
mod tests {

    use crate::shape3d::Sphere;

    use super::*;

    #[test]
    fn test_one() {
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 1.0), 1.0);
        let objects = vec![sphere];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        let res = closest_intersection(&objects, &ray, origin).unwrap();

        assert_eq!(objects[0], res.0);
        assert_eq!(Vector3D::new(0.0, 0.0, 0.0), res.1);
    }

    #[test]
    fn test_none() {
        let objects = vec![Sphere::new(Vector3D::new(3.0, 0.0, 0.0), 1.0)];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(None, closest_intersection(&objects, &ray, origin));
    }

    #[test]
    fn test_many() {
        let objects = vec![
            Sphere::new(Vector3D::new(0.0, 0.0, 1.0), 1.0),
            Sphere::new(Vector3D::new(0.0, 0.0, 2.0), 1.0),
        ];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(
            Some((objects[0], Vector3D::new(0.0, 0.0, 0.0))),
            closest_intersection(&objects, &ray, origin)
        );
    }
}