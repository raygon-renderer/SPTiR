//! Spectral Power and Reflectance Distributions

pub mod illuminant_d;
pub mod sampled;

use super::units::{SpectralRadiance, SpectralReflectance, SpectralWavelengths};

/// `$L_{\mathbf{a},\Omega,\lambda}$`, spectral radiance
pub trait SpectralPowerDistribution {
    fn radiance(&self, lambda: &SpectralWavelengths) -> SpectralRadiance;
}

/// `$R_{\lambda}$`, unitless reflectance factor
pub trait SpectralReflectanceDistribution {
    /// Computes `$R_{\lambda}$`, which should always be between 0 and 1
    fn reflectance(&self, lambda: &SpectralWavelengths) -> SpectralReflectance;
}
