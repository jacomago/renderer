use std::io::{self};

use renderer::prelude::*;

fn main() -> io::Result<()> {
    let dim = Dimensions::new(1280, 960);
    let mut image = Image::new(dim);
    let scene = Scene::new(
        vec![
            Box::new(Sphere::new(
                Vector3D::new(0.0, 0.0, 0.0),
                10.0,
                Coloring::Fill(Color::RED),
            )),
            Box::new(Sphere::new(
                Vector3D::new(00.0, 0.0, 10.0),
                20.0,
                Coloring::Fill(Color::YELLOW),
            )),
        ],
        Camera::new(
            Vector3D::new(0.0, 0.0, -20.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            10.0,
            dim,
        ),
    );
    scene.render(&mut image);
    Ppm::from(image).write(&mut io::stdout())?;
    Ok(())
}
