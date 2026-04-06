use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::UiTheme;

pub enum UiWindowId {
    CellEditor,
}

pub enum UiWindowType {
    Floating,
    Panel,
}

#[derive(Component)]
pub struct UiWindow {
    pub id: UiWindowId,
    pub window_type: UiWindowType,
}

pub fn spawn_window(
    id: UiWindowId,
    window_type: UiWindowType,
    width: Val,
    height: Val,
    ui_theme: &UiTheme,
    commands: &mut Commands,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    commands
        .spawn((
            UiWindow { id, window_type },
            Node {
                width,
                height,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: ui_theme.window.item_spacing,
                padding: ui_theme.window.padding,
                ..default()
            },
            BackgroundColor(ui_theme.window.colour),
            BorderColor::all(ui_theme.window.border_colour),
        ))
        .with_children(children)
        .id()
}
