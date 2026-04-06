use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::UiTheme;

pub fn spawn_separator(parent: &mut RelatedSpawnerCommands<ChildOf>, ui_theme: &UiTheme) {
    parent.spawn((
        Node {
            height: ui_theme.separator.height,
            width: percent(100),
            margin: ui_theme.separator.margin,
            ..default()
        },
        BackgroundColor(ui_theme.separator.colour),
    ));
}
