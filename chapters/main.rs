extern crate sptir;

use sptir::color::spaces::SRGBColor;

fn main() {
    let range = sptir::spectrum::range::SpectralRange::new(360.0, 830.0);

    println!("{:?}", range);

    /*
    let mut film = sptir::film::Film::new(128, 128);

    for y in 0..128 {
        for x in 0..128 {
            film.put(x, y, SRGBColor::new(x as f32 / 128.0, y as f32 / 128.0, 0.0).to_xyz());
        }
    }

    film.save_as("test.png", 0.1).unwrap();
    */
}
