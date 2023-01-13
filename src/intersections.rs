use crate::{
    shape3d::{RayIntersections, Sphere},
    vectors::{Ray, Vector3D},
};

pub fn closest_sphere_intersection<'a>(
    objects: &'a [Sphere],
    ray: &'a Ray,
    origin: Vector3D<f32>,
) -> Option<(&'a Sphere, Vector3D<f32>)> {
    let mut sphere_intersections: Vec<(&Sphere, Vector3D<f32>)> = objects
        .iter()
        .map(|object| (object, object.intersection(ray)))
        .filter(|(_, i)| !i.is_empty())
        .flat_map(|(o, i)| {
            i.iter()
                .map(|v| (o, *v))
                .collect::<Vec<(&Sphere, Vector3D<f32>)>>()
        })
        .collect();
    sphere_intersections.sort_by(|(_, i), (_, j)| {
        i.distance_squared(origin)
            .total_cmp(&j.distance_squared(origin))
    });
    sphere_intersections.first().map(|(s, v)| (*s, *v))
}

#[cfg(test)]
mod tests {
    use crate::color::Coloring;

    use super::*;

    #[test]
    fn test_one() {
        let objects = vec![Sphere::new(
            Vector3D::new(0.0, 0.0, 1.0),
            1.0,
            Coloring::default(),
        )];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(
            Some((&objects[0], Vector3D::new(0.0, 0.0, 0.0))),
            closest_sphere_intersection(&objects, &ray, origin)
        );
    }

    #[test]
    fn test_none() {
        let objects = vec![Sphere::new(
            Vector3D::new(3.0, 0.0, 0.0),
            1.0,
            Coloring::default(),
        )];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(None, closest_sphere_intersection(&objects, &ray, origin));
    }

    #[test]
    fn test_many() {
        let objects = vec![
            Sphere::new(Vector3D::new(0.0, 0.0, 1.0), 1.0, Coloring::default()),
            Sphere::new(Vector3D::new(0.0, 0.0, 2.0), 1.0, Coloring::default()),
        ];
        let origin = Vector3D::default();
        let ray = Ray::new(origin, Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(
            Some((&objects[0], Vector3D::new(0.0, 0.0, 0.0))),
            closest_sphere_intersection(&objects, &ray, origin)
        );
    }
}
