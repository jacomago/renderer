use colors::prelude::Color;
use geometry::prelude::{closest_intersection, Vector3D};
use image_lib::prelude::Image;

use crate::{camera::Camera, lighting::Light, object::SceneObject};

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
    camera: Camera,
}
impl Scene {
    pub fn new(objects: Vec<SceneObject>, lights: Vec<Light>, camera: Camera) -> Self {
        Self {
            objects,
            lights,
            camera,
        }
    }

    pub fn render(&self, image: &mut Image) {
        image.positions().iter().for_each(|p| {
            let new_color = self.ray_cast(p, image);

            if let Some(color) = new_color {
                image.pixel_mut(p).unwrap().color = color;
            }
        });
    }

    fn calc_lighting(
        &self,
        scene_object: &SceneObject,
        position: &Vector3D<f32>,
    ) -> Option<Color<f32>> {
        let colors = self.lights.iter().map(|l| {
            let ray = l.ray(position);
            if closest_intersection(
                &self
                    .objects
                    .iter()
                    .filter(|o| *o != scene_object)
                    .copied()
                    .collect::<Vec<SceneObject>>(),
                &ray,
                Vector3D::default(),
            )
            .is_some()
            {
                return None;
            }
            let light_color = l.surface_color(
                scene_object,
                position,
                l.angle(scene_object.shape(), position),
            );
            Some(light_color)
        });
        colors.last().unwrap_or_default()
    }

    fn ray_cast(
        &self,
        p: &geometry::prelude::Position2D<usize>,
        image: &mut Image,
    ) -> Option<Color<f32>> {
        let ray = self.camera.ray(p, image.dimensions());
        if let Some((s, intersection)) =
            closest_intersection(&self.objects, &ray, self.camera.position())
        {
            return self.calc_lighting(&s, &intersection);
        }
        None
    }
}
