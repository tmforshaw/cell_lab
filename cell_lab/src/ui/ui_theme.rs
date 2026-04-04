use bevy::prelude::*;

#[derive(Resource)]
pub struct UiTheme {
    pub panel_colour: Color,
    pub text_colour: TextColor,
    pub font: Handle<Font>,
    pub text_shadow: TextShadow,
    pub button: UiThemeButton,
    pub border: UiRect,
    pub border_radius: BorderRadius,
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
            button: UiThemeButton::default(),
            border: UiRect::axes(px(5), px(5)),
            border_radius: BorderRadius::MAX,
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
