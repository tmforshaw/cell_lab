use bevy::prelude::*;

use crate::ui::ui_theme_colour_palette::ColourPalette;

// const PALETTE: ColourPalette = ColourPalette {
//     primary: Color::linear_rgb(0.027, 0.530, 0.950),
//     secondary: Color::linear_rgb(0.135, 0.402, 0.950),
//     tertiary: Color::linear_rgb(0.328, 0.509, 0.950),
//     quaternary: Color::linear_rgb(0.553, 0.746, 0.950),
//     quinary: Color::linear_rgb(0.486, 0.114, 0.249),
// };

const PALETTE: ColourPalette = ColourPalette {
    primary: Color::linear_rgb(0.864, 0.694, 0.618),    // EFD9CE
    secondary: Color::linear_rgb(0.732, 0.527, 0.879),  // DEC0F1
    tertiary: Color::linear_rgb(0.474, 0.333, 0.847),   // B79CED
    quaternary: Color::linear_rgb(0.301, 0.212, 0.864), // 957FEF
    quinary: Color::linear_rgb(0.165, 0.119, 0.864),    // 7161EF};
};

#[derive(Resource)]
pub struct UiTheme {
    pub panel_colour: Color,
    pub text_colour: TextColor,
    pub font: Handle<Font>,
    pub text_shadow: TextShadow,
    pub border: UiRect,
    pub border_radius: BorderRadius,
    pub label_gap: Val,
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
        commands.insert_resource(Self::from_colour_palette(&asset_server, PALETTE));
    }

    #[must_use]
    pub fn from_default(asset_server: &AssetServer) -> Self {
        let font_handle = asset_server.load("fonts/fira-mono.regular.ttf");

        Self {
            panel_colour: Color::linear_rgb(0.15, 0.15, 0.18),
            text_colour: TextColor(Color::WHITE),
            font: font_handle,
            text_shadow: TextShadow::default(),
            border: UiRect::axes(px(0), px(0)),
            border_radius: BorderRadius::MAX,
            label_gap: px(5.),
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
    pub fn from_colour_palette(asset_server: &AssetServer, value: ColourPalette) -> Self {
        let font_handle = asset_server.load("fonts/fira-mono.regular.ttf");

        Self {
            panel_colour: value.quaternary,
            text_colour: TextColor(Color::WHITE),
            font: font_handle,
            text_shadow: TextShadow::default(),
            border: UiRect::axes(px(0), px(0)),
            border_radius: BorderRadius::MAX,
            label_gap: px(5.),
            heading_font_size: 30.,
            subheading_font_size: 25.,
            label_font_size: 20.,
            inner_font_size: 16.,
            window: UiThemeWindow::from(value),
            separator: UiThemeSeparator::from(value),
            semi_separator: UiThemeSemiSeparator::from(value),
            button: UiThemeButton::from(value),
            slider: UiThemeSlider::from(value),
            checkbox: UiThemeCheckbox::from(value),
            radio: UiThemeRadio::from(value),
            combobox: UiThemeCombobox::from(value),
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
    pub border_colour: Color,
    pub item_spacing: Val,
    pub padding: UiRect,
}

impl Default for UiThemeWindow {
    fn default() -> Self {
        Self {
            colour: Color::linear_rgb(0.1, 0.1, 0.1),
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
            normal_colour: value.primary.with_saturation(value.primary.saturation() * 0.9),
            hovered_colour: value.primary.with_saturation(value.primary.saturation() * 1.0),
            pressed_colour: value.primary.with_saturation(value.primary.saturation() * 0.8),
            border_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.9),
            border_hovered_colour: value.secondary.with_saturation(value.secondary.saturation() * 1.0),
            border_pressed_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.8),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeCheckbox {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.primary.with_saturation(value.primary.saturation() * 0.9),
            hovered_colour: value.primary.with_saturation(value.primary.saturation() * 1.0),
            pressed_colour: value.primary.with_saturation(value.primary.saturation() * 0.8),
            border_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.9),
            border_hovered_colour: value.secondary.with_saturation(value.secondary.saturation() * 1.0),
            border_pressed_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.8),
            normal_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.9),
            hovered_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 1.0),
            pressed_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.8),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeCombobox {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.primary.with_saturation(value.primary.saturation() * 0.9),
            hovered_colour: value.primary.with_saturation(value.primary.saturation() * 1.0),
            pressed_colour: value.primary.with_saturation(value.primary.saturation() * 0.8),
            border_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.9),
            border_hovered_colour: value.secondary.with_saturation(value.secondary.saturation() * 1.0),
            border_pressed_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.8),
            normal_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.9),
            hovered_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 1.0),
            pressed_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.8),
            normal_valuebox_colour: value.tertiary.with_saturation(value.tertiary.saturation() * 0.9),
            hovered_valuebox_colour: value.tertiary.with_saturation(value.tertiary.saturation() * 1.0),
            pressed_valuebox_colour: value.tertiary.with_saturation(value.tertiary.saturation() * 0.8),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeRadio {
    fn from(value: ColourPalette) -> Self {
        Self {
            normal_colour: value.primary.with_saturation(value.primary.saturation() * 0.9),
            hovered_colour: value.primary.with_saturation(value.primary.saturation() * 1.0),
            pressed_colour: value.primary.with_saturation(value.primary.saturation() * 0.8),
            border_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.9),
            border_hovered_colour: value.secondary.with_saturation(value.secondary.saturation() * 1.0),
            border_pressed_colour: value.secondary.with_saturation(value.secondary.saturation() * 0.8),
            normal_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.9),
            hovered_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 1.0),
            pressed_selected_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.8),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSemiSeparator {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.secondary.with_saturation(value.secondary.saturation() * 0.8),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSeparator {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.secondary.with_saturation(value.secondary.saturation() * 1.0),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeSlider {
    fn from(value: ColourPalette) -> Self {
        Self {
            track_colour: value.primary.with_saturation(value.primary.saturation() * 1.0),
            track_border_colour: value.primary.with_saturation(value.primary.saturation() * 0.8),
            handle_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.9),
            handle_hovered_colour: value.quinary.with_saturation(value.quinary.saturation() * 1.0),
            handle_pressed_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.8),
            handle_border_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.6),
            handle_hovered_border_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.7),
            handle_pressed_border_colour: value.quinary.with_saturation(value.quinary.saturation() * 0.5),
            ..default()
        }
    }
}

impl From<ColourPalette> for UiThemeWindow {
    fn from(value: ColourPalette) -> Self {
        Self {
            colour: value.quaternary.with_luminance(value.quaternary.luminance() * 0.2),
            border_colour: value.quaternary.with_luminance(value.quaternary.luminance() * 0.1),
            ..default()
        }
    }
}
