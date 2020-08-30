use super::*;

pub type SRGBColor = RGBColor<SRGB>;
pub type ACES2065_1Color = RGBColor<ACES_AP0>;
pub type ACEScgColor = RGBColor<ACES_AP1>;

#[allow(non_camel_case_types)]
pub struct ACES_AP0;
#[allow(non_camel_case_types)]
pub struct ACES_AP1;

pub struct SRGB;

impl Colorspace for ACES_AP0 {
    const PRIMARIES: RGBPrimaries = RGBPrimaries {
        r: xyY::new(0.7347, 0.2653, 1.0),
        g: xyY::new(0.0000, 1.0000, 1.0),
        b: xyY::new(0.0001, -0.077, 1.0),
    };

    const WHITEPOINT: xyY = whitepoints::ACES;
}

impl Colorspace for ACES_AP1 {
    const PRIMARIES: RGBPrimaries = RGBPrimaries {
        r: xyY::new(0.713, 0.293, 1.0),
        g: xyY::new(0.165, 0.830, 1.0),
        b: xyY::new(0.128, 0.044, 1.0),
    };

    const WHITEPOINT: xyY = whitepoints::ACES;
}

impl Colorspace for SRGB {
    const PRIMARIES: RGBPrimaries = RGBPrimaries {
        r: xyY::new(0.64, 0.33, 1.0),
        g: xyY::new(0.30, 0.60, 1.0),
        b: xyY::new(0.15, 0.06, 1.0),
    };

    const WHITEPOINT: xyY = whitepoints::D65;
}

impl SRGBColor {
    pub fn decode_transfer_function_from(srgb: &[u8]) -> SRGBColor {
        assert!(srgb.len() >= 3);

        fn decode_component(u: f32) -> f32 {
            if u < 0.04045 {
                u / 12.92
            } else {
                ((u + 0.055) / 1.055).powf(2.4)
            }
        }

        SRGBColor::new(
            decode_component(srgb[0] as f32 * 255.0),
            decode_component(srgb[1] as f32 * 255.0),
            decode_component(srgb[2] as f32 * 255.0),
        )
    }

    pub fn encode_transfer_function_to(&self, srgb: &mut [u8]) {
        assert!(srgb.len() >= 3);

        fn encode_component(u: f32) -> f32 {
            let encoded = if u <= 0.0031308 {
                12.92 * u
            } else {
                (1.055 * u).powf(1.0 / 2.4) - 0.055
            };

            encoded.min(1.0).max(0.0)
        }

        srgb[0] = (encode_component(self.r) * 255.0) as u8;
        srgb[1] = (encode_component(self.g) * 255.0) as u8;
        srgb[2] = (encode_component(self.b) * 255.0) as u8;
    }
}
