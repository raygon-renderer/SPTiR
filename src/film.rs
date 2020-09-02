use image::RgbImage;

use crate::color::spaces::{SRGBColor, SRGB};
use crate::spectrum::XYZSpectrum;

pub struct Film {
    pub pixels: Vec<XYZSpectrum>,
    pub width: usize,
    pub height: usize,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Film {
        let num_pixels = width * height;

        Film {
            pixels: vec![XYZSpectrum::new(0.0, 0.0, 0.0); num_pixels],
            width,
            height,
        }
    }

    pub fn put(&mut self, x: usize, y: usize, energy: XYZSpectrum) {
        assert!(x < self.width && y < self.height);

        let idx = self.width * y + x;

        self.pixels[idx] = self.pixels[idx] + energy;
    }

    pub fn save_as<P: AsRef<std::path::Path>>(&self, path: P, exposure: f32) -> Result<(), image::ImageError> {
        let image = RgbImage::from_fn(self.width as u32, self.height as u32, |x, y| {
            let idx = self.width * x as usize + y as usize;
            let color = self.pixels[idx].to_rgb::<SRGB>();

            let mut pixel = image::Rgb([0; 3]);
            tonemap(color, exposure).encode_transfer_function_to(&mut pixel.0);

            pixel
        });

        image.save(path)
    }
}

// Uncharted 2 tonemapper
fn tonemap(color: SRGBColor, exposure: f32) -> SRGBColor {
    const T_A: f32 = 0.15;
    const T_B: f32 = 0.50;
    const T_C: f32 = 0.10;
    const T_D: f32 = 0.20;
    const T_E: f32 = 0.02;
    const T_F: f32 = 0.30;
    const T_W: f32 = 11.2;

    const fn tonemap_component(x: f32) -> f32 {
        ((x * (T_A * x + T_C * T_B) + T_D * T_E) / (x * (T_A * x + T_B) + T_D * T_F)) - T_E / T_F
    }

    SRGBColor::new(
        tonemap_component(color.r * 16.0 * exposure),
        tonemap_component(color.g * 16.0 * exposure),
        tonemap_component(color.b * 16.0 * exposure),
    ) / tonemap_component(T_W)
}
