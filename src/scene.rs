use crate::{camera::Camera, color::Color, image::Image, shape3d::Shape3d, vectors::Vector3D};

pub struct Scene {
    objects: Vec<Box<dyn Shape3d>>,
    camera: Camera,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Shape3d>>, camera: Camera) -> Self {
        Self { objects, camera }
    }

    pub fn render(&self, image: &mut Image) {
        image.positions().iter().for_each(|p| {
            let ray = self.camera.ray(p, image.dimensions());
            let mut color_intersections: Vec<(Color, Vector3D<f32>)> = self
                .objects
                .iter()
                .map(|object| (object, object.intersection(&ray)))
                .filter(|(_, i)| !i.is_empty())
                .flat_map(|(o, i)| {
                    i.iter()
                        .map(|v| (o.color(v), *v))
                        .filter(|(o, _)| o.is_some())
                        .map(|(o, v)| (o.unwrap(), v))
                        .collect::<Vec<(Color, Vector3D<f32>)>>()
                })
                .collect();
            color_intersections.sort_by(|(_, i), (_, j)| {
                i.distance_squared(self.camera.position())
                    .total_cmp(&j.distance_squared(self.camera.position()))
            });
            if let Some((color, _)) = color_intersections.first() {
                image.pixel_mut(p).unwrap().color = *color;
            }
        });
    }
}