//! Independent Random Sampler

use rand::{Rng, SeedableRng};

use crate::sampling::{Sample1D, Sample2D, SampleDimension1D, SampleDimension2D, Sampler};

/// Independent Random Sampler
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
