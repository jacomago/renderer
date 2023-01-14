use colors::prelude::Color;
use geometry::prelude::closest_intersection;
use image_lib::prelude::Image;

use crate::{camera::Camera, colored::Colored, lighting::Light, object::SceneObject};

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

    fn ray_cast(
        &self,
        p: &geometry::prelude::Position2D<usize>,
        image: &mut Image,
    ) -> Option<Color<f32>> {
        let ray = self.camera.ray(p, image.dimensions());
        if let Some((s, intersection)) =
            closest_intersection(&self.objects, &ray, self.camera.position())
        {
            return s.color(&intersection);
        }
        None
    }
}
