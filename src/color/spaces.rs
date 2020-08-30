use super::*;

pub type SRGBColor = RGBColor<SRGB>;
pub type ACES2065_1Color = RGBColor<ACES_AP0>;
pub type ACEScgColor = RGBColor<ACES_AP1>;

/// ACES 2065-1 Extreme Wide-Gamut Color Space
#[allow(non_camel_case_types)]
pub struct ACES_AP0;
/// ACEScg Wide-Gamut Color Space
#[allow(non_camel_case_types)]
pub struct ACES_AP1;

/// Rec701 sRGB Color Space
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
    /**
    Decodes an 8-bit encoded sRGB color.

    ```math
    \gamma ^{-1}(u)={
        \begin{cases}{
            \frac {u}{12.92}} &= {\frac {25u}{323}}&u\leq 0.04045 \\
            \left({\tfrac {u+0.055}{1.055}}\right)^{2.4} &= \left({\tfrac {200u+11}{211}}\right)^{\frac {12}{5}}&{\text{otherwise}}
        \end{cases}
    }
    ```
    * where `$u$` is `$R_{srgb}, G_{srgb}, \text{or}\ B_{srgb}$`

    **NOTE**: Input slice must have a length of at least 3.
    */
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

    /**
    Encodes a linear sRGB color to an output slice of 8-bit integers.

    ```math
    \gamma (u)={
        \begin{cases}
            12.92u &= {\frac {323u}{25}}&u\leq 0.0031308 \\
            1.055u^{1/2.4}-0.055 &= {\frac {211u^{\frac {5}{12}}-11}{200}}&{\text{otherwise}}
        \end{cases}
    }
    ```
    * where `$u$` is `$R_{Linear}, G_{Linear}, \text{or}\ B_{Linear}$`

    **NOTE**: Output slice must have a length of at least 3.
    */
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
