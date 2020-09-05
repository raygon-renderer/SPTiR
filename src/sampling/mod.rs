#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample1D(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample2D {
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleDimension1D {
    Wavelength,
    Time,
    LightPick,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleDimension2D {
    Film,
    Lens,
    BSDF,
    Light,
}

pub trait Sampler {
    fn set_sample(&mut self, sample: usize);
    fn set_pixel(&mut self, x: usize, y: usize);

    fn next1d(&mut self, dim: SampleDimension1D) -> Sample1D;
    fn next2d(&mut self, dim: SampleDimension2D) -> Sample2D;
}

pub mod samplers {
    pub mod independent;
}
