use bevy::prelude::*;

use crate::ui::ui_theme_colour_palette::{BaseColourPalette, ColourPalette};

// const BASE_PALETTE: BaseColourPalette = BaseColourPalette {
//     primary: Color::linear_rgb(0.380, 0.886, 0.580),    // 61e294
//     accent: Color::linear_rgb(0.482, 0.804, 0.729),     // 7bcdba
//     background: Color::linear_rgb(0.592, 0.600, 0.792), // 9799ca
//     surface: Color::linear_rgb(0.741, 0.576, 0.847),    // bd93d8
//     text: Color::linear_rgb(0.706, 0.478, 0.918),       // b47aea
// };

// const BASE_PALETTE: BaseColourPalette = BaseColourPalette {
//     background: Color::linear_rgb(0.035, 0.040, 0.090), // deep blue-black
//     surface: Color::linear_rgb(0.080, 0.090, 0.160),    // lifted surface
//     primary: Color::linear_rgb(0.450, 0.350, 0.950),    // vivid blue
//     accent: Color::linear_rgb(0.750, 0.400, 1.000),     // strong purple
//     text: Color::linear_rgb(0.920, 0.940, 1.000),       // near-white with slight blue tint
// };

// const BASE_PALETTE: BaseColourPalette = BaseColourPalette {
//     // Core surfaces
//     background: Color::linear_rgb(0.05, 0.05, 0.1), // deep night blue
//     surface: Color::linear_rgb(0.12, 0.10, 0.20),   // dark purple panel
//     text: Color::linear_rgb(0.95, 0.95, 1.0),       // off-white

//     // Primary accent / buttons
//     primary: Color::linear_rgb(0.50, 0.35, 0.90), // bright violet
//     accent: Color::linear_rgb(0.45, 0.45, 0.95),  // soft periwinkle
// };

const BASE_PALETTE: BaseColourPalette = BaseColourPalette {
    // Core surfaces
    background: Color::linear_rgb(0.04, 0.04, 0.04), // dark gray background
    surface: Color::linear_rgb(0.1, 0.1, 0.1),       // slightly lighter gray for panels
    text: Color::linear_rgb(0.95, 0.95, 0.95),       // off-white text

    // Primary accent / interactive
    primary: Color::linear_rgb(0.55, 0.35, 0.85), // purple for buttons, highlights
    accent: Color::linear_rgb(0.20, 0.45, 0.75),  // muted blue for selection / highlights
};

#[derive(Resource)]
pub struct UiTheme {
    pub background_colour: Color,
    pub text_colour: TextColor,
    pub font: Handle<Font>,
    pub text_shadow: TextShadow,
    pub border: UiRect,
    pub border_radius: BorderRadius,
    pub label_gap: Val,
    pub heading_padding: UiRect,
    pub heading_font_size: f32,
    pub subheading_font_size: f32,
    pub label_font_size: f32,
    pub inner_font_size: f32,
    pub window: UiThemeWindow,
    pub separator: UiThemeSeparator,
    pub semi_separator: UiThemeSemiSeparator,
    pub button: UiThemeButton,
    pub slider: UiThemeSlider,
    pub checkbox: UiThemeCheckbox,
    pub radio: UiThemeRadio,
    pub combobox: UiThemeCombobox,
}

impl UiTheme {
    #[allow(clippy::needless_pass_by_value)]
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(Self::from_base_colour_palette(&asset_server, BASE_PALETTE));
    }

    #[must_use]
    pub fn from_default(asset_server: &AssetServer) -> Self {
        let font_handle = asset_server.load("fonts/fira-mono.regular.ttf");

        Self {
            background_colour: Color::BLACK,
            text_colour: TextColor(Color::WHITE),
            font: font_handle,
            text_shadow: TextShadow::default(),
            border: UiRect::axes(px(5), px(5)),
            border_radius: BorderRadius::MAX,
            label_gap: px(5.),
            heading_padding: UiRect::all(px(10)),
            heading_font_size: 30.,
            subheading_font_size: 25.,
            label_font_size: 20.,
            inner_font_size: 16.,
            window: UiThemeWindow::default(),
            separator: UiThemeSeparator::default(),
            semi_separator: UiThemeSemiSeparator::default(),
            button: UiThemeButton::default(),
            slider: UiThemeSlider::default(),
            checkbox: UiThemeCheckbox::default(),
            radio: UiThemeRadio::default(),
            combobox: UiThemeCombobox::default(),
        }
    }

    #[must_use]
    pub fn from_base_colour_palette(asset_server: &AssetServer, base_palette: BaseColourPalette) -> Self {
        let palette = ColourPalette::from(base_palette);

        let default = Self::from_default(asset_server);

        Self {
            // Colours
            background_colour: palette.background,
            text_colour: TextColor(palette.text_primary),

            // Derive subsections using palette as well
            window: UiThemeWindow::from(palette),
            separator: UiThemeSeparator::from(palette),
            semi_separator: UiThemeSemiSeparator::from(palette),
            button: UiThemeButton::from(palette),
            slider: UiThemeSlider::from(palette),
            checkbox: UiThemeCheckbox::from(palette),
            radio: UiThemeRadio::from(palette),
            combobox: UiThemeCombobox::from(palette),

            // Fill in other fields
            ..default
        }
    }
}

pub struct UiThemeButton {
    pub normal_colour: Color,
    pub hovered_colour: Color,
    pub pressed_colour: Color,
    pub border_colour: Color,
    pub border_hovered_colour: Color,
    pub border_pressed_colour: Color,
    pub padding: UiRect,
}

impl Default for UiThemeButton {
    fn default() -> Self {
        Self {
            normal_colour: Color::linear_rgb(0.25, 0.25, 0.3),
            hovered_colour: Color::linear_rgb(0.35, 0.35, 0.4),
            pressed_colour: Color::linear_rgb(0.2, 0.2, 0.3),
            border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            border_hovered_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            border_pressed_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(10), px(5)),
        }
    }
}

pub struct UiThemeSlider {
    pub width: Val,
    pub height: Val,
    pub track_colour: Color,
    pub track_border_colour: Color,
    pub handle_width: Val,
    pub handle_height: Val,
    pub handle_colour: Color,
    pub handle_hovered_colour: Color,
    pub handle_pressed_colour: Color,
    pub handle_border_colour: Color,
    pub handle_hovered_border_colour: Color,
    pub handle_pressed_border_colour: Color,
    pub padding: UiRect,
}

impl Default for UiThemeSlider {
    fn default() -> Self {
        Self {
            width: px(200),
            height: px(3),
            track_colour: Color::linear_rgb(1., 0., 1.),
            track_border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            handle_width: px(16.),
            handle_height: px(16.),
            handle_colour: Color::linear_rgb(1.0, 1., 1.),
            handle_hovered_colour: Color::linear_rgb(0.8, 0.8, 0.8),
            handle_pressed_colour: Color::linear_rgb(0.6, 0.6, 0.6),
            handle_border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            handle_hovered_border_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            handle_pressed_border_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(0), px(3)),
        }
    }
}

pub struct UiThemeCheckbox {
    pub normal_colour: Color,
    pub hovered_colour: Color,
    pub pressed_colour: Color,
    pub normal_selected_colour: Color,
    pub hovered_selected_colour: Color,
    pub pressed_selected_colour: Color,
    pub border_colour: Color,
    pub border_hovered_colour: Color,
    pub border_pressed_colour: Color,
    pub padding: UiRect,
}

impl Default for UiThemeCheckbox {
    fn default() -> Self {
        Self {
            normal_colour: Color::linear_rgb(0.25, 0.25, 0.3),
            hovered_colour: Color::linear_rgb(0.35, 0.35, 0.4),
            pressed_colour: Color::linear_rgb(0.2, 0.2, 0.3),
            normal_selected_colour: Color::linear_rgb(0.5, 0., 0.5),
            hovered_selected_colour: Color::linear_rgb(0.4, 0., 0.4),
            pressed_selected_colour: Color::linear_rgb(0.2, 0., 0.2),
            border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            border_hovered_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            border_pressed_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(7.5), px(7.5)),
        }
    }
}

pub struct UiThemeRadio {
    pub normal_colour: Color,
    pub hovered_colour: Color,
    pub pressed_colour: Color,
    pub normal_selected_colour: Color,
    pub hovered_selected_colour: Color,
    pub pressed_selected_colour: Color,
    pub border_colour: Color,
    pub border_hovered_colour: Color,
    pub border_pressed_colour: Color,
    pub padding: UiRect,
    pub option_padding: UiRect,
    pub option_spacing: Val,
}

impl Default for UiThemeRadio {
    fn default() -> Self {
        Self {
            normal_colour: Color::linear_rgb(0.25, 0.25, 0.3),
            hovered_colour: Color::linear_rgb(0.35, 0.35, 0.4),
            pressed_colour: Color::linear_rgb(0.2, 0.2, 0.3),
            normal_selected_colour: Color::linear_rgb(0.5, 0., 0.5),
            hovered_selected_colour: Color::linear_rgb(0.4, 0., 0.4),
            pressed_selected_colour: Color::linear_rgb(0.2, 0., 0.2),
            border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            border_hovered_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            border_pressed_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(2.5), px(2.5)),
            option_padding: UiRect::axes(px(7.5), px(7.5)),
            option_spacing: px(5.),
        }
    }
}

pub struct UiThemeCombobox {
    pub normal_colour: Color,
    pub hovered_colour: Color,
    pub pressed_colour: Color,
    pub normal_selected_colour: Color,
    pub hovered_selected_colour: Color,
    pub pressed_selected_colour: Color,
    pub normal_valuebox_colour: Color,
    pub hovered_valuebox_colour: Color,
    pub pressed_valuebox_colour: Color,
    pub border_colour: Color,
    pub border_hovered_colour: Color,
    pub border_pressed_colour: Color,
    pub padding: UiRect,
    pub option_padding: UiRect,
    pub option_spacing: Val,
}

impl Default for UiThemeCombobox {
    fn default() -> Self {
        Self {
            normal_colour: Color::linear_rgb(0.25, 0.25, 0.3),
            hovered_colour: Color::linear_rgb(0.35, 0.35, 0.4),
            pressed_colour: Color::linear_rgb(0.2, 0.2, 0.3),
            normal_selected_colour: Color::linear_rgb(0.5, 0., 0.5),
            hovered_selected_colour: Color::linear_rgb(0.4, 0., 0.4),
            pressed_selected_colour: Color::linear_rgb(0.2, 0., 0.2),
            normal_valuebox_colour: Color::linear_rgb(0.5, 0.5, 0.),
            hovered_valuebox_colour: Color::linear_rgb(0.4, 0.4, 0.),
            pressed_valuebox_colour: Color::linear_rgb(0.2, 0.2, 0.),
            border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            border_hovered_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            border_pressed_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(2.5), px(2.5)),
            option_padding: UiRect::axes(px(7.5), px(7.5)),
            option_spacing: px(5.),
        }
    }
}

pub struct UiThemeWindow {
    pub colour: Color,
    pub colour_variant: Color,
    pub border_colour: Color,
    pub item_spacing: Val,
    pub padding: UiRect,
}

impl Default for UiThemeWindow {
    fn default() -> Self {
        Self {
            colour: Color::linear_rgb(0.1, 0.1, 0.1),
            colour_variant: Color::linear_rgb(0.15, 0.15, 0.15),
            border_colour: Color::linear_rgb(0.05, 0.05, 0.05),
            item_spacing: px(5),
            padding: UiRect::axes(px(10), px(10)),
        }
    }
}

pub struct UiThemeSeparator {
    pub height: Val,
    pub colour: Color,
    pub margin: UiRect,
}

impl Default for UiThemeSeparator {
    fn default() -> Self {
        Self {
            height: px(5),
            colour: Color::linear_rgb(0., 0., 0.),
            margin: UiRect::axes(px(0.), px(10.)),
        }
    }
}

pub struct UiThemeSemiSeparator {
    pub height: Val,
    pub colour: Color,
    pub margin: UiRect,
}

impl Default for UiThemeSemiSeparator {
    fn default() -> Self {
        Self {
            height: px(2.5),
            colour: Color::linear_rgb(0.025, 0.025, 0.025),
            margin: UiRect::axes(px(0.), px(5.)),
        }
    }
}

impl From<ColourPalette> for UiThemeButton {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.interactive,
            hovered_colour: value.interactive_hovered,
            pressed_colour: value.interactive_pressed,
            border_colour: value.border,
            border_hovered_colour: value.border_hovered,
            border_pressed_colour: value.border_pressed,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeCheckbox {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.interactive,
            hovered_colour: value.interactive_hovered,
            pressed_colour: value.interactive_pressed,
            border_colour: value.border,
            border_hovered_colour: value.border_hovered,
            border_pressed_colour: value.border_pressed,
            normal_selected_colour: value.accent,
            hovered_selected_colour: value.accent_hovered,
            pressed_selected_colour: value.accent_pressed,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeCombobox {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.interactive,
            hovered_colour: value.interactive_hovered,
            pressed_colour: value.interactive_pressed,
            border_colour: value.border,
            border_hovered_colour: value.border_hovered,
            border_pressed_colour: value.border_pressed,
            normal_selected_colour: value.accent,
            hovered_selected_colour: value.accent_hovered,
            pressed_selected_colour: value.accent_pressed,
            normal_valuebox_colour: value.surface_accent,
            hovered_valuebox_colour: value.surface_accent_hovered,
            pressed_valuebox_colour: value.surface_accent_pressed,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeRadio {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.interactive,
            hovered_colour: value.interactive_hovered,
            pressed_colour: value.interactive_pressed,
            border_colour: value.border,
            border_hovered_colour: value.border_hovered,
            border_pressed_colour: value.border_pressed,
            normal_selected_colour: value.accent,
            hovered_selected_colour: value.accent_hovered,
            pressed_selected_colour: value.accent_pressed,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSemiSeparator {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.separator_subtle,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSeparator {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.separator,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSlider {
    fn from(value: ColourPalette) -> Self {
        Self {
            track_colour: value.surface_variant,
            track_border_colour: value.border,
            handle_colour: value.interactive,
            handle_hovered_colour: value.interactive_hovered,
            handle_pressed_colour: value.interactive_pressed,
            handle_border_colour: value.border,
            handle_hovered_border_colour: value.border_hovered,
            handle_pressed_border_colour: value.border_pressed,
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeWindow {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.surface,
            colour_variant: value.surface_variant,
            border_colour: value.border_strong,
            ..default()
        }
    }
}
