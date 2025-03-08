use image::{Rgb, save_buffer};
use std::ops::{Index, IndexMut};
use std::path::Path;

/// Custom image class with pixel-level control
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Rgb<f64>>, // Stores RGB values
    to_srgb: bool,
}

impl Image {
    /// Create a new image with given dimensions
    pub fn new(width: u32, height: u32, defalut_color: Rgb<f64>, to_srgb: bool) -> Self {
        Image {
            width,
            height,
            pixels: vec![defalut_color; (width * height) as usize],
            to_srgb,
        }
    }

    /// Get pixel values at (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgb<f64> {
        self.validate_coordinates(x, y);
        self.pixels[self.pixel_index(x, y)]
    }

    /// Set pixel values at (x, y)
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb<f64>) {
        self.validate_coordinates(x, y);
        let index = self.pixel_index(x, y);
        self.pixels[index] = color;
    }

    /// Iterate over pixels with coordinates (x, y, &color)
    pub fn enumerate_pixels(&mut self) -> impl Iterator<Item = (u32, u32, &mut Rgb<f64>)> {
        // Capture width/height by value first
        let width = self.width;
        
        self.pixels.iter_mut().enumerate().map(move |(index, pixel)| {
            // Use captured values instead of self
            let x = (index % width as usize) as u32;
            let y = (index / width as usize) as u32;
            (x, y, pixel)
        })
    }
    

    /// Save the image to a file
    pub fn save(&self, path: &Path) -> Result<(), image::ImageError> {
        let buffer: Vec<u8>;
        
        if self.to_srgb {
            buffer = self.pixels
            .iter()
            .flat_map(|p| p.0.iter().map(|&v| {
                (v.clamp(0.0, 1.0).powf(1.0/2.2) * 255.0).round() as u8
            }))
            .collect();
        } else {
            buffer = self.pixels
            .iter()
            .flat_map(|p| p.0.iter().map(|&v| {
                (v.clamp(0.0, 1.0) * 255.0).round() as u8
            }))
            .collect();
        }

        save_buffer(
            Path::new(path),
            &buffer,
            self.width,
            self.height,
            image::ColorType::Rgb8,
        )
    }

    // Helper function
    fn pixel_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn validate_coordinates(&self, x: u32, y: u32) {
        assert!(x < self.width, "Invalid coordinates");
        assert!(y < self.height, "Invalid coordinates");
    }

    // Property accessors
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
}

impl Index<(u32, u32)> for Image {
    type Output = Rgb<f64>;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        self.validate_coordinates(x, y);
        &self.pixels[self.pixel_index(x, y)]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        self.validate_coordinates(x, y);
        let index = self.pixel_index(x, y);
        &mut self.pixels[index]
    }
}
