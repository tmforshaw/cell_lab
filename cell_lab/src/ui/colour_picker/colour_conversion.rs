use bevy::prelude::*;

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn rgba_to_hsva(colour: Color) -> (f32, f32, f32, f32) {
    // Break into linear RGB components
    let [r, g, b, a] = colour.to_linear().to_f32_array();

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    // Hue
    #[allow(clippy::float_cmp)]
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    // Saturation (HSV)
    let s = if max == 0.0 { 0.0 } else { delta / max };

    // Value
    let v = max;

    (h, s, v, a)
}

#[must_use]
pub fn rgb_to_hsv(colour: Color) -> (f32, f32, f32) {
    let (h, s, v, _) = rgba_to_hsva(colour);

    (h, s, v)
}
