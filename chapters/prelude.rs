/*!

# Spectral Path Tracing in Rust: Prelude

Ray tracing is all the hype right now, both in realtime and production movie renderers. It's a simple and efficient way to
explore and scene and simulate light through it.

Path tracing is a specific application of ray tracing in conjuction with Monte-Carlo
integration to simulate light transport for realistic global illumination and materials, naturally
simulating many effects such as soft shadows, depth of field, motion blur, ambient occlusion and indirect loghting.

However, many current path tracing renderers have a fatal flaw: **RGB Color**.

Your computer monitor displays pixels using a combination of three subpixels for Red, Green and Blue. Likewise,
most images, image editing software, video players, web browsers, etc. all use RGB color to manipulate and display
content. That's all mostly fine.

However, when simulating the complex physical interactions between materials and light,
forcing color into three strict bins becomes not only awkward, but incorrect.

The ACEScg color space (discussed later) attempts to rememdy this somewhat with a wide-gamut colorspace for rendering within,
but even that fails to capture the real and subtle interactions across imperfect spectra.

This is where **Spectral Rendering** comes in. Spectral Rendering simulates true individual
wavelengths of light and the associated spectral flux (energy) at each wavelength.

For example, consider this raw cone response curve to the D65 standard illuminant and then the XYZ color values of it, derived by integrating the cone responses.

<div style="text-align:center" class="flex-container">
    <div class="flex-child">
        <img src="../sptir_assets/d65-raw.png" alt="D65 Raw" style="align:center"/>
        <figcaption>Fig. 1: Tristimulus Response of the D65 Illuminant</figcaption>
    </div>
    <div class="flex-child">
        <img src="../sptir_assets/d65-xyx.png" alt="D65 XYZ" style="align:center"/>
        <figcaption>Fig. 2: XYZ integral of the D65 Illuminant</figcaption>
    </div>
</div>

As you can see, fitting such wide and varied functions into three bins for all of human vision loses quite a bit of information.

Human vision is inherently lossy, and rendering using lossy descriptions of the real world will inevitably produce inferior and unrealistic results.

With spectral rendering, however, we sidestep such approximations entirely by tracing individual wavelengths of light and energy, giving unprecedented color realism.

*/

#[doc(hidden)]
pub fn main() {
    println!("Hello, world!");
}
