use crate::prelude::Vector3D;

pub trait Normal<T> {
    fn normal(&self, p: Vector3D<T>) -> Vector3D<T>;
}
