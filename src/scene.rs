use crate::{
    camera::Camera,
    image::Image,
    shape::Colored,
    shape3d::{RayIntersections, Sphere},
    vectors::{Ray, Vector3D},
};

pub struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
}

fn closest_sphere_intersection<'a>(
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
