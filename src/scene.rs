use crate::{camera::Camera, image::Image, shape3d::Shape3d};

pub struct Scene {
    objects: Vec<Box<dyn Shape3d>>,
    camera: Camera,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Shape3d>>, camera: Camera) -> Self {
        Self { objects, camera }
    }

    pub fn render(&self) -> Image {
        Image::new(self.camera.screen.size.into())
    }
}
