use std::ops::{Add, Mul};

use crate::color::{Colorspace, RGBColor};

use numeric_array::{typenum, NumericArray};
use typenum::Unsigned;

pub mod distribution;

/// Defines how many wavelengths should be used for HWSS
pub type NumLanes = typenum::consts::U4;

/// Helper type to define an array of values correspoding to each sampled wavelength
pub type Lanes = NumericArray<f32, NumLanes>;

/// Hero wavelength spectrum samples
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralWavelengthSamples {
    /// The sampled wavelengths (`$\lambda$`), where `$\lambda_h = \lambda_0$`
    pub lambda: Lanes,

    /// The sampling probability for this wavelength
    ///
    /// If the sampling is uniform, then this is simply `$\small\frac{1}{\lambda_{max} - \lambda_{min}}$`
    pub pdf: Lanes,
}

impl SpectralWavelengthSamples {
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

/// XYZ Tristimulus color values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZSpectrum {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZSpectrum {
    pub const ZERO: XYZSpectrum = XYZSpectrum::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> XYZSpectrum {
        XYZSpectrum { x, y, z }
    }

    /// Converts XYZ tristimulus values to RGB within a certain colorspace.
    ///
    /// NOTE: This alone does no chromatic adaptation, and conversion from rgb to rgb of different colorspaces
    /// should be handled with [`RGBColor::convert`](`RGBColor::convert`)
    ///
    /// [`RGBColor::convert`]: crate::color::RGBColor::convert
    pub fn to_rgb<CS: Colorspace>(self) -> RGBColor<CS> {
        let XYZSpectrum { x: r, y: g, z: b } = CS::XYZ_TO_RGB.transform_xyz(self);
        RGBColor::new(r, g, b)
    }

    /**
        Calculates the tristimulus response values for the given wavelength in nanometers using
        a simple gaussian approximation.

        ```math
        g(x;\alpha ,\mu ,\sigma _{1},\sigma _{2})=\alpha \exp \left({\frac {(x-\mu )^{2}}{-2\sigma ^{2}}}\right),
        \\ {\text{ where }}\sigma ={\begin{cases}\sigma _{1},&x<\mu ,\\\sigma _{2},&x\geq \mu .\end{cases}}
        ```
        then, the response values can be calculated as:
        ```math
        \begin{aligned}
            {\overline {x}}(\lambda )&=g(\lambda ;1.056,5998,379,310)+g(\lambda ;0.362,4420,160,267)+g(\lambda ;-0.065,5011,204,262), \\
            {\overline {y}}(\lambda )&=g(\lambda ;0.821,5688,469,405)+g(\lambda ;0.286,5309,163,311),\\
            {\overline {z}}(\lambda )&=g(\lambda ;1.217,4370,118,360)+g(\lambda ;0.681,4590,260,138).
        \end{aligned}
        ```
        where `$\lambda$` is measured in Angstroms. However, the conversion from nanometers to angstroms is handled automatically here.
    */
    #[rustfmt::skip]
    pub fn from_wavelength(lambda: f32) -> XYZSpectrum {
        #[inline]
        fn gaussian(lambda: f32, alpha: f32, mu: f32, sigma1: f32, sigma2: f32) -> f32 {
            let sqrt = (lambda - mu) / if lambda < mu { sigma1 } else { sigma2 };

            alpha * (-0.5 * (sqrt * sqrt)).exp()
        }

        let angstroms_lambda = lambda * 10.0;

        let x = gaussian(angstroms_lambda,  1.056, 5998.0, 379.0, 310.0)
              + gaussian(angstroms_lambda,  0.362, 4420.0, 160.0, 267.0)
              + gaussian(angstroms_lambda, -0.065, 5011.0, 204.0, 262.0);

        let y = gaussian(angstroms_lambda,  0.821, 5688.0, 469.0, 405.0)
              + gaussian(angstroms_lambda,  0.286, 5309.0, 163.0, 311.0);

        let z = gaussian(angstroms_lambda,  1.217, 4370.0, 118.0, 360.0)
              + gaussian(angstroms_lambda,  0.681, 4590.0, 260.0, 138.0);

        XYZSpectrum::new(x, y, z)
    }
}

impl Add for XYZSpectrum {
    type Output = XYZSpectrum;
    fn add(mut self, rhs: XYZSpectrum) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

/// `$\int_{\lambda_{min}}^{\lambda_{max}} \overline{y}(\lambda) d\lambda$`, where wavelengths can be sampled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralRange {
    /// `$\lambda_{min}$`, the minimum wavelength which can be sampled
    pub min: f32,
    /// `$\lambda_{max}$`, the maximum wavelength which can be sampled
    pub max: f32,
    /// Normalizing factor
    ///
    /// ```math
    /// \int_{\lambda_{min}}^{\lambda_{max}} \overline{y}(\lambda) d\lambda
    /// ```
    pub y_integral: f32,
}

impl SpectralRange {
    pub fn new(min: f32, max: f32) -> SpectralRange {
        SpectralRange {
            min,
            max,
            y_integral: crate::math::integrate(min, max, 128, |lambda| XYZSpectrum::from_wavelength(lambda).y),
        }
    }

    /**
        ```math
            \large
            \begin{aligned}
                X &= \frac{1}{C} \sum_{j=0}^C \left( \frac{1}{N} \int_{\lambda}\frac{L_{\mathrm{e}, \Omega, \lambda}(\lambda_j)}{p(\lambda_j^s)}\,{\overline {x}}(\lambda )\,d\lambda \right) \\
                Y &= \frac{1}{C} \sum_{j=0}^C \left( \frac{1}{N} \int_{\lambda}\frac{L_{\mathrm{e}, \Omega, \lambda}(\lambda_j)}{p(\lambda_j^s)}\,{\overline {y}}(\lambda )\,d\lambda \right) \\
                Z &= \frac{1}{C} \sum_{j=0}^C \left( \frac{1}{N} \int_{\lambda}\frac{L_{\mathrm{e}, \Omega, \lambda}(\lambda_j)}{p(\lambda_j^s)}\,{\overline {z}}(\lambda )\,d\lambda \right)

                \\ \text{where} \\
                N &= \int _{\lambda }I(\lambda )\,{\overline {y}}(\lambda )\,d\lambda
            \end{aligned}
        ```
        and `$p(\lambda_j^s)$` is the probability density function value of each sampled wavelength.

        If the sampling is uniform, then `$p(\lambda_j^s)$` is simply `$\small\frac{1}{\lambda_{max} - \lambda_{min}}$`
    */
    pub fn hero_to_xyz(&self, wl: SpectralWavelengthSamples, flux: SpectralFlux) -> XYZSpectrum {
        let mut avg = XYZSpectrum::ZERO;

        for i in 0..NumLanes::USIZE {
            let denom = self.y_integral * wl.pdf[i] * NumLanes::USIZE as f32;

            if denom.is_normal() {
                let xyz = XYZSpectrum::from_wavelength(wl.lambda[i]);
                let normalized_energy = flux.energy[i] / denom;

                avg.x += xyz.x * normalized_energy;
                avg.y += xyz.y * normalized_energy;
                avg.z += xyz.z * normalized_energy;
            }
        }

        avg
    }

    /**
        Samples a hero wavelength and `NumLanes - 1` number of equidistantly spaced other wavelength samples
        within the defined range, using a rotation function `$r_j: \Lambda \rarr \Lambda$`:
        ```math
            r_j \left( \lambda_{h} \right)=\left( \lambda_h - \lambda_{min} + \frac{j}{C} \overline{\lambda}\right)\textbf{mod}\ \overline{\lambda} + \lambda_{min}
        ```
        where
        ```math
        \begin{aligned}
            \overline{\lambda} &= \lambda_{max}-\lambda_{min} \\
            \lambda_h          &= t\overline{\lambda} + \lambda_{min} \\
            j                  &= 1,\dots,C \\
            t                  &\in [0,1)
        \end{aligned}
        ```
    */
    pub fn sample(&self, t: f32) -> SpectralWavelengthSamples {
        let lambda_bar = self.max - self.min;

        let hero = self.min + t.min(1.0).max(0.0) * lambda_bar; // basically a lerp

        let mut lambda = Lanes::splat(hero);

        for j in 1..NumLanes::USIZE {
            let jf = j as f32;

            lambda[j] = self.min + (hero - self.min + lambda_bar * jf / NumLanes::USIZE as f32) % lambda_bar;
        }

        SpectralWavelengthSamples {
            lambda,
            pdf: Lanes::splat(1.0 / lambda_bar),
        }
    }
}
