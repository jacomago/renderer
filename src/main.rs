use std::{
    fs::File,
    io::{self},
};

use renderer::prelude::*;

fn write_file(filename: &str, ppm: &Ppm) -> io::Result<()> {
    let mut file = File::create(filename)?;
    ppm.write(&mut file)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut image = Image::from_w_h(64, 48);
    let circle = Circle::new(5.0, Position::new(32.0, 24.0), Color::WHITE);

    circle.draw(&mut image);
    println!("{}", image);
    write_file("circle.ppm", &image.into())?;
    Ok(())
}
