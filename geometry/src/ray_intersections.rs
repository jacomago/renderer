use crate::prelude::{Ray, Vector3D};

pub trait RayIntersections {
    fn intersection(&self, ray: &Ray) -> Vec<Vector3D<f32>>;
}
