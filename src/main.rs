use std::io::{self};

use renderer::prelude::*;

fn main() -> io::Result<()> {
    let mut image = Image::from_w_h(128, 96);
    let circle = Circle::new(
        50.0,
        Position::new(32.0, 24.0),
        Coloring::gradient((Color::RED, Color::YELLOW)),
    );

    circle.draw(&mut image);
    Ppm::from(image).write(&mut io::stdout())?;
    Ok(())
}
