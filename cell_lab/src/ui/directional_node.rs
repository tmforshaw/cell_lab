use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::UiTheme;

pub fn spawn_horizontal(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    ui_theme: &UiTheme,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>) -> Option<Entity>,
) -> Option<Entity> {
    let mut root_builder = parent.spawn(Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        column_gap: ui_theme.window.item_spacing,
        ..default()
    });

    // Capture the returned entity from the closure
    let mut child_entity = None;
    root_builder.with_children(|parent| child_entity = children(parent));

    child_entity
}

pub fn spawn_vertical(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    ui_theme: &UiTheme,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>) -> Option<Entity>,
) -> Option<Entity> {
    let mut root_builder = parent.spawn(Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: ui_theme.window.item_spacing,
        ..default()
    });

    // Capture the returned entity from the closure
    let mut child_entity = None;
    root_builder.with_children(|parent| child_entity = children(parent));

    child_entity
}
