use colors::prelude::Color;
use geometry::prelude::{instersections, Normal, Ray, Vector3D};

use crate::object::SceneObject;

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
    pub fn color(&self) -> &Color<f32> {
        &self.color
    }
    pub fn ray(&self, position: &Vector3D<f32>) -> Ray {
        Ray::new(self.position, self.position - *position)
    }

    pub fn angle(&self, shape: &dyn Normal<f32>, position: &Vector3D<f32>) -> f32 {
        let r = self.position - *position;
        let normalized_r = r.normalize();
        normalized_r.dot(&shape.normal(position))
    }

    pub fn light_ray_intersect_other_object(
        &self,
        objects: &[SceneObject],
        position: &Vector3D<f32>,
        scene_object: &SceneObject,
    ) -> bool {
        let ray = self.ray(position);
        if instersections(
            &objects
                .iter()
                .filter(|o| *o != scene_object)
                .copied()
                .collect::<Vec<SceneObject>>(),
            &ray,
        )
        .is_empty()
        {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use colors::prelude::Coloring;
    use geometry::prelude::Sphere;

    use super::*;

    #[test]
    fn test_light_ray_intersect_none() {
        let object = SceneObject::new(Sphere::new(Vector3D::default(), 1.0), Coloring::default());
        let light = Light::new(
            Vector3D::new(1.0, 1.0, 1.0),
            Color::default(),
            Color::default(),
        );
        let position = Vector3D::default();

        assert!(!light.light_ray_intersect_other_object(&[object], &position, &object));
    }

    #[test]
    fn test_light_ray_intersect_one() {
        let object = SceneObject::new(Sphere::new(Vector3D::default(), 1.0), Coloring::default());
        let intersection_object = SceneObject::new(
            Sphere::new(Vector3D::new(2.0, 0.0, 0.0), 1.0),
            Coloring::default(),
        );
        let light = Light::new(
            Vector3D::new(3.0, 0.0, 0.0),
            Color::default(),
            Color::default(),
        );
        let position = Vector3D::default();

        assert!(light.light_ray_intersect_other_object(
            &[object, intersection_object],
            &position,
            &object
        ));
    }
}
