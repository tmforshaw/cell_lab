use bevy::prelude::*;

pub struct BaseColourPalette {
    pub background: Color,
    pub surface: Color,
    pub primary: Color,
    pub accent: Color,
    pub text: Color,
}

fn overlay(base: Color, tint: Color, alpha: f32) -> Color {
    // Perform linear interpolation of the colours
    let [base_r, base_g, base_b, base_a] = base.to_linear().to_f32_array();
    let [tint_r, tint_g, tint_b, tint_a] = tint.to_linear().to_f32_array();

    // Clamp here to allow for extrapolation
    Color::linear_rgba(
        (tint_r - base_r).mul_add(alpha, base_r).clamp(0., 1.),
        (tint_g - base_g).mul_add(alpha, base_g).clamp(0., 1.),
        (tint_b - base_b).mul_add(alpha, base_b).clamp(0., 1.),
        (tint_a - base_a).mul_add(alpha, base_a).clamp(0., 1.),
    )
}

#[derive(Debug, Copy, Clone)]
pub struct ColourPalette {
    // Surfaces
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,

    // Text
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_disabled: Color,

    // Interaction
    pub interactive: Color,
    pub interactive_hovered: Color,
    pub interactive_pressed: Color,
    pub interactive_secondary: Color,

    // Acccent / Selection
    pub accent: Color,
    pub accent_hovered: Color,
    pub accent_pressed: Color,

    // Borderes
    pub border: Color,
    pub border_hovered: Color,
    pub border_pressed: Color,
    pub border_strong: Color,

    // Separators
    pub separator: Color,
    pub separator_subtle: Color,

    // Surface Accents
    pub surface_accent: Color,
    pub surface_accent_hovered: Color,
    pub surface_accent_pressed: Color,
}

impl From<BaseColourPalette> for ColourPalette {
    fn from(base: BaseColourPalette) -> Self {
        // constants
        const HOVERED_LIGHTEN: f32 = 0.06;
        const PRESSED_DARKEN: f32 = 0.12;
        const SURFACE_LIFT: f32 = -0.4; // negative overlay to move slightly away from background
        const BORDER_BLEND: f32 = 0.5;

        // precompute border base
        let base_border = overlay(base.surface, base.background, BORDER_BLEND);

        Self {
            // Surfaces
            background: base.background,
            surface: base.surface,
            surface_variant: overlay(base.surface, base.background, SURFACE_LIFT),

            // Text
            text_primary: base.text,
            text_secondary: overlay(base.text, base.background, 0.4),
            text_disabled: overlay(base.text, base.background, 0.7),

            // Interactive elements
            interactive: base.primary,
            interactive_hovered: overlay(base.primary, base.background, HOVERED_LIGHTEN),
            interactive_pressed: overlay(base.primary, base.background, -PRESSED_DARKEN),
            interactive_secondary: overlay(base.primary, base.surface, 0.5),

            // Accents
            accent: base.accent,
            accent_hovered: overlay(base.accent, base.background, HOVERED_LIGHTEN),
            accent_pressed: overlay(base.accent, base.background, -PRESSED_DARKEN),

            // Borders
            border: base_border,
            border_hovered: overlay(base_border, base.background, HOVERED_LIGHTEN),
            border_pressed: overlay(base_border, base.background, -PRESSED_DARKEN),
            border_strong: overlay(base_border, base.background, -PRESSED_DARKEN * 2.0),

            // Separators
            separator: overlay(base.surface, base.background, -0.3),
            separator_subtle: overlay(base.surface, base.background, -0.15),

            // Surface accents
            surface_accent: overlay(base.surface, base.accent, 0.2),
            surface_accent_hovered: overlay(base.surface, base.accent, 0.3),
            surface_accent_pressed: overlay(base.surface, base.accent, 0.4),
        }
    }
}
