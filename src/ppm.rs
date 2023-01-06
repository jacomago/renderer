use std::{
    fs::File,
    io::{self, Write},
};

use crate::image::Dimensions;

pub struct Ppm {
    dim: Dimensions,
    max_color_value: u8,
    bytes: Vec<u8>,
}

impl Ppm {
    pub fn new(dim: Dimensions, max_color_value: u8, bytes: Vec<u8>) -> Self {
        Self {
            dim,
            max_color_value,
            bytes,
        }
    }

    pub fn write(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", self.dim.width, self.dim.height)?;
        writeln!(file, "{}", self.max_color_value)?;
        file.write_all(&self.bytes)?;
        Ok(())
    }
}
