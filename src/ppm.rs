use std::io::{self, Write};

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

    pub fn write(&self, output: &mut dyn Write) -> io::Result<()> {
        writeln!(output, "P6")?;
        writeln!(output, "{} {}", self.dim.width, self.dim.height)?;
        writeln!(output, "{}", self.max_color_value)?;
        output.write_all(&self.bytes)?;
        Ok(())
    }
}
