use rand::{Rng, SeedableRng};

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

pub struct IndependentSampler {
    sample: usize,
    x: usize,
    y: usize,
    rng: rand_xoshiro::Xoroshiro128Plus,
}

impl IndependentSampler {
    pub fn new(seed: u64) -> IndependentSampler {
        IndependentSampler {
            sample: 0,
            x: 0,
            y: 0,
            rng: SeedableRng::seed_from_u64(seed),
        }
    }
}

impl Sampler for IndependentSampler {
    fn set_sample(&mut self, sample: usize) {
        self.sample = sample;
    }

    fn set_pixel(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn next1d(&mut self, _dim: SampleDimension1D) -> Sample1D {
        Sample1D(self.rng.gen())
    }

    fn next2d(&mut self, _dim: SampleDimension2D) -> Sample2D {
        Sample2D {
            u: self.rng.gen(),
            v: self.rng.gen(),
        }
    }
}
