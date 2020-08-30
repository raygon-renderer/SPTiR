use crate::spectrum::XYZSpectrum;

pub struct Film {
    pub pixels: Vec<XYZSpectrum>,
    pub width: usize,
    pub height: usize,
}
