use std::fmt::Display;

use crate::{color::Color, position::Position, prelude::Ppm};

#[derive(Debug, Default, Clone)]
pub struct Pixel {
    pub color: Color,
}

impl Pixel {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
    pub fn rgb_bytes(&self) -> Vec<u8> {
        self.color.rgb_bytes()
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color.monotone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Image {
    dim: Dimensions,
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(dim: Dimensions) -> Self {
        Self {
            dim,
            pixels: vec![vec![Pixel::default(); dim.width]; dim.height],
        }
    }
    pub fn from_w_h(w: usize, h: usize) -> Self {
        Self::new(Dimensions::new(w, h))
    }
    pub fn pixel(&self, position: &Position<usize>) -> Option<&Pixel> {
        match self.pixels.get(position.y) {
            Some(row_pixels) => row_pixels.get(position.x),
            None => None,
        }
    }
    pub fn pixel_mut(&mut self, position: &Position<usize>) -> Option<&mut Pixel> {
        match self.pixels.get_mut(position.y) {
            Some(row_pixels) => row_pixels.get_mut(position.x),
            None => None,
        }
    }

    pub fn positions(&self) -> Vec<Position<usize>> {
        (0..self.dim.height)
            .flat_map(|col| (0..self.dim.width).map(move |row| Position::new(row, col)))
            .collect()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        self.pixels.iter().for_each(|v_p| {
            v_p.iter().for_each(|p| {
                output.push_str(&format!("{}", p));
            });
            output.push('\n');
        });
        write!(f, "{}", output)
    }
}

impl From<Image> for Ppm {
    fn from(image: Image) -> Self {
        Self::new(
            image.dim,
            255,
            image
                .pixels
                .iter()
                .flat_map(|r_p| r_p.iter().flat_map(|p| p.rgb_bytes()))
                .collect(),
        )
    }
}
