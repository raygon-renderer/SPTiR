extern crate sptir;

use image::{Rgb, RgbImage};

fn main() {
    let width = 1024;
    let height = 512;

    let d65 = sptir::spectrum::distributions::illuminant_d::DSeries::new(6504.0).unwrap();

    //let sine = sptir::spectrum::sampled::PiecewiseLinearSpectrum::from_fn(300.0, 890.0, 128, |x| (x / 20.0).sin() * 0.5 + 0.5);

    let image = RgbImage::from_fn(width, height, |x, y| {
        let u = x as f32 / width as f32;
        let v = 1.0 - y as f32 / height as f32;

        let min = 300.0;
        let max = 830.0;

        let lambda = u * (max - min) + min;

        let power = d65.sample(lambda) / 150.0;

        if (v - power).abs() < 0.01 {
            Rgb([0; 3])
        } else {
            Rgb([255; 3])
        }
    });

    image.save("d65.png").unwrap();
}
