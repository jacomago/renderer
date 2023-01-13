use crate::{
    camera::Camera, image::Image, intersections::closest_sphere_intersection, shape::Colored,
    shape3d::Sphere,
};

pub struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
}
impl Scene {
    pub fn new(objects: Vec<Sphere>, camera: Camera) -> Self {
        Self { objects, camera }
    }

    pub fn render(&self, image: &mut Image) {
        image.positions().iter().for_each(|p| {
            let ray = self.camera.ray(p, image.dimensions());
            if let Some((s, intersection)) =
                closest_sphere_intersection(&self.objects, &ray, self.camera.position())
            {
                if let Some(color) = s.color(&intersection) {
                    image.pixel_mut(p).unwrap().color = color;
                }
            }
        });
    }
}
