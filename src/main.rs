#![feature(const_fn, const_panic)]

/*!
Spectral Path Tracing in Rust
=============================




*/

pub mod color;
pub mod film;
pub mod geometry;
pub mod math;
pub mod spectrum;

fn main() {
    let range = spectrum::SpectralRange::new(360.0, 830.0);

    println!("Hello, world! {:?}", range);
}
