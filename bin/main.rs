extern crate sptir;

fn main() {
    let range = sptir::spectrum::SpectralRange::new(360.0, 830.0);

    println!("Hello, world! {:?}", range);
}
