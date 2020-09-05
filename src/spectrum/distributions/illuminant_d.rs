//! Standard Illuminant D-series

mod tables {
    pub const D_SERIES_START: f32 = 300.0;
    pub const D_SERIES_END: f32 = 830.0;

    pub const NUM_SAMPLES: usize = 54;

    pub const S0: [f32; NUM_SAMPLES] = [
        0.04, 6.00, 29.60, 55.30, 57.30, 61.80, 61.50, 68.80, 63.40, 65.80, 94.80, 104.80, 105.90, 96.80, 113.90, 125.60, 125.50, 121.30, 121.30,
        113.50, 113.10, 110.80, 106.50, 108.80, 105.30, 104.40, 100.00, 96.00, 95.10, 89.10, 90.50, 90.30, 88.40, 84.00, 85.10, 81.90, 82.60, 84.90,
        81.30, 71.90, 74.30, 76.40, 63.30, 71.70, 77.00, 65.20, 47.70, 68.60, 65.00, 66.00, 61.00, 53.30, 58.90, 61.90,
    ];

    pub const S1: [f32; NUM_SAMPLES] = [
        0.02, 4.50, 22.40, 42.00, 40.60, 41.60, 38.00, 42.40, 38.50, 35.00, 43.40, 46.30, 43.90, 37.10, 36.70, 35.90, 32.60, 27.90, 24.30, 20.10,
        16.20, 13.20, 8.60, 6.10, 4.20, 1.90, 0.00, -1.60, -3.50, -3.50, -5.80, -7.20, -8.60, -9.50, -10.90, -10.70, -12.00, -14.00, -13.60, -12.00,
        -13.30, -12.90, -10.60, -11.60, -12.20, -10.20, -7.80, -11.20, -10.40, -10.60, -9.70, -8.30, -9.30, -9.80,
    ];

    pub const S2: [f32; NUM_SAMPLES] = [
        0.0, 2.0, 4.0, 8.5, 7.8, 6.7, 5.3, 6.1, 3.0, 1.2, -1.1, -0.5, -0.7, -1.2, -2.6, -2.9, -2.8, -2.6, -2.6, -1.8, -1.5, -1.3, -1.2, -1.0, -0.5,
        -0.3, 0.0, 0.2, 0.5, 2.1, 3.2, 4.1, 4.7, 5.1, 6.7, 7.3, 8.6, 9.8, 10.2, 8.3, 9.6, 8.5, 7.0, 7.6, 8.0, 6.7, 5.2, 7.4, 6.8, 7.0, 6.4, 5.5, 6.1,
        6.5,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DSeries {
    m_1: f32,
    m_2: f32,
}

impl DSeries {
    pub fn new(temperature: f32) -> Option<DSeries> {
        let x_d = if temperature < 4000.0 || temperature > 25000.0 {
            return None;
        } else {
            let t = temperature;
            let t2 = t * t;
            let t3 = t2 * t;

            if temperature < 7000.0 {
                -4.607e9 / t3 + 2.9678e6 / t2 + 0.09911e3 / t + 0.244063
            } else {
                -2.00064e9 / t3 + 1.9016e6 / t2 + 0.24748e3 / t + 0.23704
            }
        };

        let y_d = -3.0 * x_d * x_d + 2.87 * x_d - 0.275;

        let m = 0.0241 + 0.2562 * x_d - 0.7341 * y_d;

        let m_1 = (0.03 - 31.4424 * x_d + 30.0717 * y_d) / m;
        let m_2 = (-1.3515 - 1.7703 * x_d + 5.9114 * y_d) / m;

        Some(DSeries { m_1, m_2 })
    }

    pub fn sample(&self, lambda: f32) -> f32 {
        if lambda < tables::D_SERIES_START || lambda > tables::D_SERIES_END {
            return 0.0;
        }

        let idx = ((lambda.floor() - tables::D_SERIES_START) / 10.0) as usize;

        let t = (lambda * 0.1).fract();
        let t1 = 1.0 - t;

        let s0 = tables::S0[idx] * t1 + tables::S0[idx + 1] * t;
        let s1 = tables::S1[idx] * t1 + tables::S1[idx + 1] * t;
        let s2 = tables::S2[idx] * t1 + tables::S2[idx + 1] * t;

        s0 + self.m_1 * s1 + self.m_2 * s2
    }
}

use crate::spectrum::units::{Lanes, SpectralRadiance, SpectralWavelengths};

impl super::SpectralPowerDistribution for DSeries {
    fn radiance(&self, lambda: &SpectralWavelengths) -> SpectralRadiance {
        let mut energy = Lanes::splat(0.0);

        for (lambda, radiance) in lambda.lambda.iter().zip(&mut energy) {
            *radiance = self.sample(*lambda);
        }

        SpectralRadiance { energy }
    }
}
