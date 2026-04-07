use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::UiTheme;

pub fn spawn_label<S: AsRef<str>>(parent: &mut RelatedSpawnerCommands<ChildOf>, text: S, ui_theme: &UiTheme) {
    parent.spawn((
        Text::new(text.as_ref()),
        TextFont {
            font: ui_theme.font.clone(),
            font_size: ui_theme.label_font_size,
            ..default()
        },
        ui_theme.text_colour,
        ui_theme.text_shadow,
    ));
}

pub fn spawn_heading<S: AsRef<str>>(parent: &mut RelatedSpawnerCommands<ChildOf>, text: S, ui_theme: &UiTheme) {
    parent
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                padding: ui_theme.heading_padding,
                ..default()
            },
            BackgroundColor(ui_theme.window.colour_variant),
        ))
        .with_child((
            Text::new(text.as_ref()),
            TextFont {
                font: ui_theme.font.clone(),
                font_size: ui_theme.heading_font_size,
                ..default()
            },
            ui_theme.text_colour,
            ui_theme.text_shadow,
        ));
}

pub fn spawn_subheading<S: AsRef<str>>(parent: &mut RelatedSpawnerCommands<ChildOf>, text: S, ui_theme: &UiTheme) {
    parent.spawn((
        Text::new(text.as_ref()),
        TextFont {
            font: ui_theme.font.clone(),
            font_size: ui_theme.subheading_font_size,
            ..default()
        },
        ui_theme.text_colour,
        ui_theme.text_shadow,
    ));
}
