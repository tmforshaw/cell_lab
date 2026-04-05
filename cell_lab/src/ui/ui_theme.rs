use bevy::prelude::*;

#[derive(Resource)]
pub struct UiTheme {
    pub panel_colour: Color,
    pub text_colour: TextColor,
    pub font: Handle<Font>,
    pub text_shadow: TextShadow,
    pub border: UiRect,
    pub border_radius: BorderRadius,
    pub button: UiThemeButton,
    pub slider: UiThemeSlider,
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
            border: UiRect::axes(px(5.), px(5.)),
            border_radius: BorderRadius::MAX,
            button: UiThemeButton::default(),
            slider: UiThemeSlider::default(),
        });
    }
}

pub struct UiThemeButton {
    pub normal_colour: Color,
    pub hover_colour: Color,
    pub pressed_colour: Color,
    pub border_colour: Color,
    pub border_hover_colour: Color,
    pub border_pressed_colour: Color,
    pub padding: UiRect,
}

impl Default for UiThemeButton {
    fn default() -> Self {
        Self {
            border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            border_hover_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            border_pressed_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            normal_colour: Color::linear_rgb(0.25, 0.25, 0.3),
            hover_colour: Color::linear_rgb(0.35, 0.35, 0.4),
            pressed_colour: Color::linear_rgb(0.2, 0.2, 0.3),
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
    pub handle_hover_colour: Color,
    pub handle_pressed_colour: Color,
    pub handle_border_colour: Color,
    pub handle_hover_border_colour: Color,
    pub handle_pressed_border_colour: Color,
    pub padding: UiRect,
}

impl Default for UiThemeSlider {
    fn default() -> Self {
        Self {
            width: px(200),
            height: px(20),
            track_colour: Color::linear_rgb(1., 0., 1.),
            track_border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            handle_width: px(25.),
            handle_height: px(25.),
            handle_colour: Color::linear_rgb(1.0, 1., 1.),
            handle_hover_colour: Color::linear_rgb(0.8, 0.8, 0.8),
            handle_pressed_colour: Color::linear_rgb(0.6, 0.6, 0.6),
            handle_border_colour: Color::linear_rgb(0.1, 0.1, 0.1),
            handle_hover_border_colour: Color::linear_rgb(0.2, 0.2, 0.2),
            handle_pressed_border_colour: Color::linear_rgb(0.4, 0.4, 0.4),
            padding: UiRect::axes(px(20), px(5)),
        }
    }
}
