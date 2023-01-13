use crate::{
    shape3d::RayIntersections,
    vectors::{Ray, Vector3D},
};

pub fn closest_sphere_intersection<'a>(
    objects: &'a Vec<Box<dyn RayIntersections>>,
    ray: &'a Ray,
    origin: Vector3D<f32>,
) -> Option<(&'a Box<dyn RayIntersections>, Vector3D<f32>)> {
    objects
        .iter()
        .map(|object| (object, object.intersection(ray)))
        .filter(|(_, i)| !i.is_empty())
        .flat_map(|(o, i)| {
            i.iter()
                .map(|v| (o, *v))
                .collect::<Vec<(&Box<dyn RayIntersections>, Vector3D<f32>)>>()
        })
        .min_by(|(_, i), (_, j)| {
            i.distance_squared(origin)
                .total_cmp(&j.distance_squared(origin))
        })
}

#[cfg(test)]
mod tests {

    use crate::shape3d::Sphere;

    use super::*;

    #[test]
    fn test_one() {
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 1.0), 1.0);
        let objects = vec![Box::new(sphere)];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        let res = closest_sphere_intersection(&objects, &ray, origin);

        assert_eq!(Some((&objects[0], Vector3D::new(0.0, 0.0, 0.0))),);
    }

    #[test]
    fn test_none() {
        let objects = vec![Sphere::new(Vector3D::new(3.0, 0.0, 0.0), 1.0)];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(None, closest_sphere_intersection(&objects, &ray, origin));
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
            Some((&objects[0], Vector3D::new(0.0, 0.0, 0.0))),
            closest_sphere_intersection(&objects, &ray, origin)
        );
    }
}
