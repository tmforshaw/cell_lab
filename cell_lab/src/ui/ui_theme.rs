use bevy::prelude::*;

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
    pub button: UiThemeButton,
    pub slider: UiThemeSlider,
    pub checkbox: UiThemeCheckbox,
    pub radio: UiThemeRadio,
    pub combobox: UiThemeCombobox,
}

impl UiTheme {
    #[allow(clippy::needless_pass_by_value)]
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font_handle = asset_server.load("fonts/fira-mono.regular.ttf");

        commands.insert_resource(Self {
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
            button: UiThemeButton::default(),
            slider: UiThemeSlider::default(),
            checkbox: UiThemeCheckbox::default(),
            radio: UiThemeRadio::default(),
            combobox: UiThemeCombobox::default(),
        });
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
            padding: UiRect::axes(px(7.5), px(7.5)),
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
            padding: UiRect::axes(px(7.5), px(7.5)),
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
