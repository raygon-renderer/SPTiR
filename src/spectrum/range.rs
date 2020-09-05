//! Range `$\int_{\lambda_{min}}^{\lambda_{max}} \overline{y}(\lambda) d\lambda$`, where wavelengths can be sampled.

use numeric_array::typenum::Unsigned;

use super::units::{Lanes, NumLanes, SpectralFlux, SpectralWavelengths};
use super::xyz::XYZSpectrum;

/// Range `$\int_{\lambda_{min}}^{\lambda_{max}} \overline{y}(\lambda) d\lambda$`, where wavelengths can be sampled.
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
    /// Precomputed visible wavelength range between 360 and 840 nanometers.
    pub const VISIBLE: SpectralRange = SpectralRange {
        min: 360.0,
        max: 840.0,
        // Note that this integral value is for the Gaussian approximation used here, not the reference CIE 1931 basis.
        y_integral: 106.922074243,
    };

    /**
        Creates a new spectral range between `$\lambda_{min}$` and `$\lambda_{max}$`, automatically computing:

        ```math
            \int_{\lambda_{min}}^{\lambda_{max}} \overline{y}(\lambda) d\lambda
        ```
    */
    #[rustfmt::skip]
    pub fn new(min: f32, max: f32) -> SpectralRange {
        SpectralRange {
            min,
            max,
            y_integral: crate::math::integrate(min, max, 1e-6, 8,
                |lambda| super::xyz::y_bar(lambda * 10.0)),
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
    pub fn hero_to_xyz(&self, wl: SpectralWavelengths, flux: SpectralFlux) -> XYZSpectrum {
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
    pub fn sample(&self, t: f32) -> SpectralWavelengths {
        let lambda_bar = self.max - self.min;

        let hero = self.min + t.min(1.0).max(0.0) * lambda_bar; // basically a lerp

        let mut lambda = Lanes::splat(hero);

        for j in 1..NumLanes::USIZE {
            let jf = j as f32;

            lambda[j] = self.min + (hero - self.min + lambda_bar * jf / NumLanes::USIZE as f32) % lambda_bar;
        }

        SpectralWavelengths {
            lambda,
            pdf: Lanes::splat(1.0 / lambda_bar),
        }
    }
}
