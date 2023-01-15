use colors::prelude::Color;
use geometry::prelude::{instersections, Normal, Ray, Vector3D};

use crate::{
    colored::Colored,
    prelude::{Light, SceneObject},
};

fn angle(ray: &Ray, shape: &dyn Normal<f32>, position: &Vector3D<f32>) -> f32 {
    let r = ray.direction;
    let normalized_r = r.normalize();
    normalized_r.dot(&shape.normal(position))
}

fn ray_intersect_other_object(
    ray: &Ray,
    objects: &[SceneObject],
    scene_object: &SceneObject,
) -> bool {
    if instersections(
        &objects
            .iter()
            .filter(|o| *o != scene_object)
            .copied()
            .collect::<Vec<SceneObject>>(),
        ray,
    )
    .is_empty()
    {
        return false;
    }
    true
}

pub fn light_bounce_color(
    objects: &[SceneObject],
    light: &Light,
    position: &Vector3D<f32>,
    scene_object: &SceneObject,
) -> Option<Color<f32>> {
    let ray = light.ray(position);
    if ray_intersect_other_object(&ray, objects, scene_object) {
        return None;
    }
    let light_color = surface_color(
        light,
        scene_object,
        position,
        angle(&ray, scene_object.shape(), position),
    );
    Some(light_color)
}

fn surface_color(
    light: &Light,
    shape: &dyn Colored<Vector3D<f32>>,
    position: &Vector3D<f32>,
    surface_coeff: f32,
) -> Color<f32> {
    let shape_color = shape.color(position).unwrap_or_default();
    *light.color() * shape_color * surface_coeff
}

#[cfg(test)]
mod tests {
    use colors::prelude::Coloring;
    use geometry::prelude::Sphere;

    use super::*;

    #[test]
    fn test_ray_intersect_none() {
        let object = SceneObject::new(Sphere::new(Vector3D::default(), 1.0), Coloring::default());
        let ray = Ray::new(Vector3D::new(1.0, 0.0, 0.0), Vector3D::new(-1.0, 0.0, 0.0));

        assert!(!ray_intersect_other_object(&ray, &[object], &object));
    }

    #[test]
    fn test_ray_intersect_one() {
        let object = SceneObject::new(Sphere::new(Vector3D::default(), 1.0), Coloring::default());
        let intersection_object = SceneObject::new(
            Sphere::new(Vector3D::new(2.0, 0.0, 0.0), 1.0),
            Coloring::default(),
        );
        let ray = Ray::new(Vector3D::new(3.0, 0.0, 0.0), Vector3D::new(-1.0, 0.0, 0.0));

        assert!(ray_intersect_other_object(
            &ray,
            &[object, intersection_object],
            &object
        ));
    }
}
