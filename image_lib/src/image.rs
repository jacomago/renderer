use std::fmt::Display;

use colors::prelude::Color;
use geometry::prelude::{Dimensions, Position2D};

use crate::ppm::Ppm;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pixel<T> {
    pub color: Color<T>,
}

impl<T> Pixel<T> {
    pub fn new(color: Color<T>) -> Self {
        Self { color }
    }
}

impl Pixel<f32> {
    pub fn rgb_bytes(&self) -> Vec<u8> {
        std::convert::Into::<Color<u8>>::into(self.color).rgb_bytes()
    }
}

impl Display for Pixel<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color.monotone())
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    dim: Dimensions<usize>,
    pixels: Vec<Vec<Pixel<f32>>>,
}

impl Image {
    pub fn new(dim: Dimensions<usize>) -> Self {
        Self {
            dim,
            pixels: vec![vec![Pixel::default(); dim.w()]; dim.h()],
        }
    }
    pub fn dimensions(&self) -> Dimensions<usize> {
        self.dim
    }
    pub fn from_pixels(pixels: Vec<Vec<Pixel<f32>>>) -> Self {
        Self {
            dim: Dimensions::new(pixels.first().unwrap().len(), pixels.len()),
            pixels,
        }
    }
    pub fn from_w_h(w: usize, h: usize) -> Self {
        Self::new(Dimensions::new(w, h))
    }
    pub fn pixel(&self, position: &Position2D<usize>) -> Option<&Pixel<f32>> {
        match self.pixels.get(position.y) {
            Some(row_pixels) => row_pixels.get(position.x),
            None => None,
        }
    }
    pub fn pixel_mut(&mut self, position: &Position2D<usize>) -> Option<&mut Pixel<f32>> {
        match self.pixels.get_mut(position.y) {
            Some(row_pixels) => row_pixels.get_mut(position.x),
            None => None,
        }
    }

    pub fn positions(&self) -> Vec<Position2D<usize>> {
        self.dim.positions()
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
