//! Spectral Units (Wavelength `$\lambda_h$`, Flux `$\Phi_{\mathbf{e},\lambda}$`, Radiance `$L_{\mathbf{a},\Omega,\lambda}$`, Reflectance `$R_{\lambda}$`)

use std::ops::{Add, Mul};

use numeric_array::{typenum, NumericArray, NumericConstant};

/// Defines how many wavelengths should be used for HWSS
pub type NumLanes = typenum::consts::U4;

/// Helper type to define an array of values correspoding to each sampled wavelength
pub type Lanes = NumericArray<f32, NumLanes>;

/// `$\lambda_h$` Hero wavelength spectrum samples with PDF
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralWavelengths {
    /// The sampled wavelengths (`$\lambda$`), where `$\lambda_h = \lambda_0$`
    pub lambda: Lanes,

    /// The sampling probability for this wavelength
    ///
    /// If the sampling is uniform, then this is simply `$\small\frac{1}{\lambda_{max} - \lambda_{min}}$`
    pub pdf: Lanes,
}

impl SpectralWavelengths {
    /// Hero wavelength `$\lambda_h$`
    pub fn hero(&self) -> f32 {
        self.lambda[0]
    }
}

/// `$L_{\mathbf{a},\Omega,\lambda}$`, surface radiance in watts per steradian per sqr-metre, per nanometer
pub struct SpectralRadiance {
    pub energy: Lanes,
}

impl Add for SpectralRadiance {
    type Output = SpectralRadiance;
    fn add(mut self, rhs: Self) -> SpectralRadiance {
        self.energy += rhs.energy;
        self
    }
}

impl Mul<f32> for SpectralRadiance {
    type Output = SpectralFlux;
    fn mul(self, rhs: f32) -> SpectralFlux {
        SpectralFlux {
            energy: self.energy * NumericConstant(rhs),
        }
    }
}

/// `$R_{\lambda}$`, unitless reflectance factor between 0 and 1
pub struct SpectralReflectance {
    pub refl: Lanes,
}

impl Mul for SpectralReflectance {
    type Output = SpectralReflectance;
    fn mul(mut self, rhs: Self) -> SpectralReflectance {
        self.refl *= rhs.refl;
        self
    }
}

/// `$\Phi_{\mathbf{e},\lambda}$`, radiant energy in watts per metre
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralFlux {
    pub energy: Lanes,
}

impl Mul<SpectralReflectance> for SpectralFlux {
    type Output = SpectralFlux;
    fn mul(mut self, rhs: SpectralReflectance) -> SpectralFlux {
        self.energy *= rhs.refl;
        self
    }
}

impl Add for SpectralFlux {
    type Output = SpectralFlux;
    fn add(mut self, rhs: Self) -> SpectralFlux {
        self.energy += rhs.energy;
        self
    }
}

impl Mul<f32> for SpectralFlux {
    type Output = SpectralFlux;
    fn mul(mut self, rhs: f32) -> SpectralFlux {
        self.energy *= NumericConstant(rhs);
        self
    }
}
