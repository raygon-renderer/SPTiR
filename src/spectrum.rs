/// Defines how many wavelengths should be used for HWSS
pub const NUM_LANES: usize = 4;

/// Helper type to define how many wavelength samples are taken at once
pub type Lanes = [f32; NUM_LANES];

/// Hero-Wavelength Spectrum Sample
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeroWavelengthSample {
    /// The sampled wavelengths (`$\lambda$`)
    pub lambda: Lanes,
    /// The Radiant Flux (`$\Phi_{\mathrm{e}}$`) being carried at each wavelength
    pub energy: Lanes,
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralRange {
    pub min: f32,
    pub max: f32,
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

    /// ```math
    /// X=\sum_{j=0}^{C} \left({\frac {1}{N}}\int _{\lambda }S_j(\lambda )\,I(\lambda )\,{\overline {x}}(\lambda )\,d\lambda \right) \\
    /// Y=\sum_{j=0}^{C} \left({\frac {1}{N}}\int _{\lambda }S_j(\lambda )\,I(\lambda )\,{\overline {y}}(\lambda )\,d\lambda \right) \\
    /// Z=\sum_{j=0}^{C} \left({\frac {1}{N}}\int _{\lambda }S_j(\lambda )\,I(\lambda )\,{\overline {z}}(\lambda )\,d\lambda \right) \\
    /// ```
    /// where
    /// ```math
    /// N=\int _{\lambda }I(\lambda )\,{\overline {y}}(\lambda )\,d\lambda
    /// ```
    pub fn hero_to_xyz(&self, hero: HeroWavelengthSample) -> XYZSpectrum {
        let mut sum = XYZSpectrum::ZERO;

        for (lambda, energy) in hero.lambda.iter().zip(&hero.energy) {
            let xyz = XYZSpectrum::from_wavelength(*lambda);
            let normalized_energy = energy / self.y_integral;

            sum.x += normalized_energy * xyz.x;
            sum.y += normalized_energy * xyz.y;
            sum.z += normalized_energy * xyz.z;
        }

        sum
    }

    /// Samples a hero wavelength and `NUM_LANES` number of equidistantly spaced other wavelength samples
    /// within the defined range, using a rotation function `$r_j: \Lambda \rarr \Lambda$`:
    /// ```math
    /// r_j \left( \lambda_{h} \right)=\left( \lambda_h - \lambda_{min} + \frac{j}{C} \overline{\lambda}\right)\textbf{mod}\ \overline{\lambda} + \lambda_{min}
    /// ```
    /// where
    /// ```math
    /// \begin{aligned}
    /// j &= 1,\dots,C \\ \overline{\lambda}&=\lambda_{max}-\lambda_{min}
    /// \end{aligned}
    /// ```
    pub fn sample_hero(&self, t: f32) -> HeroWavelengthSample {
        let lambda_bar = self.max - self.min;

        let hero = self.min + t * lambda_bar;

        let mut lambda = [hero; NUM_LANES];

        for j in 1..NUM_LANES {
            let jf = j as f32;

            lambda[j] = self.min + (hero - self.min + lambda_bar * jf / NUM_LANES as f32) % lambda_bar;
        }

        HeroWavelengthSample {
            lambda,
            energy: [0.0; NUM_LANES],
        }
    }
}

impl HeroWavelengthSample {
    pub fn hero(&self) -> f32 {
        self.lambda[0]
    }
}
