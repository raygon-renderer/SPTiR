use crate::spectrum::{SpectralRadiance, SpectralReflectance, SpectralWavelengthSamples};

pub trait SpectralPowerDistribution {
    fn radiance(&self, lambda: SpectralWavelengthSamples) -> SpectralRadiance;
}

pub trait SpectralReflectanceDistribution {
    fn reflectance(&self, lambda: SpectralWavelengthSamples) -> SpectralReflectance;
}
