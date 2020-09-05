//! Arbitrary sampled spectral distributions

use float_ord::FloatOrd;

pub struct PiecewiseLinearSpectrum {
    pub lambda_y: Vec<(f32, f32)>,
}

impl PiecewiseLinearSpectrum {
    pub fn new(mut lambda_y: Vec<(f32, f32)>) -> PiecewiseLinearSpectrum {
        lambda_y.sort_unstable_by_key(|x| FloatOrd(x.0));

        PiecewiseLinearSpectrum { lambda_y }
    }

    pub fn from_fn(lambda_min: f32, lambda_max: f32, samples: usize, f: impl Fn(f32) -> f32) -> PiecewiseLinearSpectrum {
        let delta = lambda_max - lambda_min;

        let mut res = vec![(0.0, 0.0); samples];

        for (i, y) in res.iter_mut().enumerate() {
            let t = i as f32 / samples as f32;
            let lambda = t * delta + lambda_min;

            *y = (lambda, f(lambda));
        }

        PiecewiseLinearSpectrum { lambda_y: res }
    }

    pub fn sample(&self, lambda: f32) -> f32 {
        match self.lambda_y.binary_search_by_key(&FloatOrd(lambda), |x| FloatOrd(x.0)) {
            Ok(idx) | Err(idx) => {
                if idx == 0 {
                    self.lambda_y[0].1
                } else if idx >= self.lambda_y.len() {
                    self.lambda_y[self.lambda_y.len() - 1].1
                } else {
                    let (la, ya) = self.lambda_y[idx - 1];
                    let (lb, yb) = self.lambda_y[idx];

                    let t = (lambda - la) / (lb - la);

                    ya * (1.0 - t) + yb * t
                }
            }
        }
    }
}

use crate::spectrum::units::{Lanes, SpectralRadiance, SpectralReflectance, SpectralWavelengths};

impl super::SpectralPowerDistribution for PiecewiseLinearSpectrum {
    fn radiance(&self, lambda: &SpectralWavelengths) -> SpectralRadiance {
        let mut energy = Lanes::splat(0.0);

        for (lambda, radiance) in lambda.lambda.iter().zip(&mut energy) {
            *radiance = self.sample(*lambda);
        }

        SpectralRadiance { energy }
    }
}

impl super::SpectralReflectanceDistribution for PiecewiseLinearSpectrum {
    fn reflectance(&self, lambda: &SpectralWavelengths) -> SpectralReflectance {
        let mut refl = Lanes::splat(0.0);

        for (lambda, reflectance) in lambda.lambda.iter().zip(&mut refl) {
            *reflectance = self.sample(*lambda).max(0.0).min(1.0); // clamp between 0.0 and 1.0
        }

        SpectralReflectance { refl }
    }
}
