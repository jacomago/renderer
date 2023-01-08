use crate::{
    dimensions::Dimensions,
    vectors::{Position2D, Ray, Vector3D},
};

pub struct Direction {
    up: Vector3D<f32>,
    right: Vector3D<f32>,
}

impl Direction {
    pub fn new(up: Vector3D<f32>, right: Vector3D<f32>) -> Self {
        Self { up, right }
    }
}
pub struct Screen {
    size: Dimensions<usize>,
    position: Vector3D<f32>,
    direction: Direction,
}

impl Screen {
    pub fn new(size: Dimensions<usize>, position: Vector3D<f32>, direction: Direction) -> Self {
        Self {
            size,
            position,
            direction,
        }
    }
    // Maps from pixel dimensions to screen dimensions
    // adding 0.5 to be in centre of pixel
    fn screen_position(
        &self,
        position: &Position2D<usize>,
        dimensions: Dimensions<usize>,
    ) -> Vector3D<f32> {
        self.position
            + self.direction.right
                * (position.x as f32 + 0.5 - self.size.w() as f32 / 2.0)
                * (self.size.w() as f32 / dimensions.w() as f32)
            + self.direction.up
                * (position.y as f32 + 0.5 - self.size.h() as f32 / 2.0)
                * (self.size.h() as f32 / dimensions.h() as f32)
    }
}

pub struct Camera {
    position: Vector3D<f32>,
    screen: Screen,
}
fn screen_position(focal_distance: f32) -> Vector3D<f32> {
    Vector3D::new(0.0, 0.0, focal_distance)
}
impl Camera {
    pub fn new(
        position: Vector3D<f32>,
        up: Vector3D<f32>,
        right: Vector3D<f32>,
        focal_distance: f32,
        screen_dim: Dimensions<usize>,
    ) -> Self {
        Self {
            position,
            screen: Screen::new(
                screen_dim,
                screen_position(focal_distance),
                Direction::new(up, right),
            ),
        }
    }

    pub fn ray(&self, position: &Position2D<usize>, dimensions: Dimensions<usize>) -> Ray {
        let screen_position = self.screen.screen_position(position, dimensions);
        let d = screen_position - self.position;
        Ray::new(self.position, d)
    }

    pub fn position(&self) -> Vector3D<f32> {
        self.position
    }

    pub fn size(&self) -> Dimensions<usize> {
        self.screen.size
    }
}

#[cfg(test)]
mod tests {
    use crate::vectors::Vector3D;

    use super::*;

    #[test]
    fn test_ray() {
        let camera = Camera::new(
            Vector3D::new(0.0, 0.0, -4.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            1.0,
            Dimensions::new(5, 5),
        );
        let ray = camera.ray(&Position2D::new(2, 2), Dimensions::new(5, 5));
        assert_eq!(
            Ray::new(Vector3D::new(0.0, 0.0, -4.0), Vector3D::new(0.0, 0.0, 5.0),),
            ray
        );
    }
}
