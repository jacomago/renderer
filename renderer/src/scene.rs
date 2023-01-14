use geometry::prelude::closest_intersection;
use image_lib::prelude::Image;

use crate::{camera::Camera, colored::Colored, object::SceneObject};

pub struct Scene {
    objects: Vec<SceneObject>,
    camera: Camera,
}
impl Scene {
    pub fn new(objects: Vec<SceneObject>, camera: Camera) -> Self {
        Self { objects, camera }
    }

    pub fn render(&self, image: &mut Image) {
        image.positions().iter().for_each(|p| {
            let ray = self.camera.ray(p, image.dimensions());
            if let Some((s, intersection)) =
                closest_intersection(&self.objects, &ray, self.camera.position())
            {
                if let Some(color) = s.color(&intersection) {
                    image.pixel_mut(p).unwrap().color = color;
                }
            }
        });
    }
}