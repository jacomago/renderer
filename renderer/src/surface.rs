use colors::prelude::Color;
use geometry::prelude::Vector3D;

use crate::{
    colored::Colored,
    prelude::{Light, SceneObject},
};

pub fn light_bounce_color(
    objects: &[SceneObject],
    l: &Light,
    position: &Vector3D<f32>,
    scene_object: &SceneObject,
) -> Option<Color<f32>> {
    if l.light_ray_intersect_other_object(objects, position, scene_object) {
        return None;
    }
    let light_color = surface_color(
        l,
        scene_object,
        position,
        l.angle(scene_object.shape(), position),
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
