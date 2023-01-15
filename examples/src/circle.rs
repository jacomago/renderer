use std::io;

use colors::prelude::{Color, Coloring};
use geometry::prelude::{Circle, Position2D};
use image_lib::prelude::{Image, Ppm};
use renderer::prelude::{ColoredCircle, Drawable};

fn main() -> io::Result<()> {
    let mut image = Image::from_w_h(64, 48);
    let circle = ColoredCircle::new(
        Circle::new(5.0, Position2D::new(32.0, 24.0)),
        Coloring::Fill(Color::WHITE),
    );

    circle.draw(&mut image);
    Ppm::from(image).write(&mut io::stdout())?;
    Ok(())
}
