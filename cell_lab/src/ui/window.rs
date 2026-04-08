use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use strum::EnumIter;

use crate::ui::UiTheme;

#[derive(Component, Debug, Copy, Clone, EnumIter, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum UiWindowId {
    CellEditorPanel,
    SaveGenomeDialog,
    OverwriteGenomeDialog,
    SaveFilenameIsEmptyDialog,
    LoadGenomeDialog,
    DeleteGenomeDialog,
    ReplaceModeWithDefaultDialog,
    LoadDefaultGenomeDialog,
    AgeSliderFloating,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct UiWindowPanel(pub UiPanelType);

#[derive(Component, Debug, Copy, Clone)]
pub struct UiWindowDialog;

#[derive(Component, Debug, Copy, Clone)]
pub struct UiWindowFloating;

#[derive(Debug, Copy, Clone)]
pub enum UiPanelType {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn spawn_panel(
    id: UiWindowId,
    panel_type: UiPanelType,
    size: Val,
    ui_theme: &UiTheme,
    commands: &mut Commands,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    commands
        .spawn((
            // Root Node
            Node {
                width: percent(100),
                height: percent(100),
                align_items: match panel_type {
                    UiPanelType::Left | UiPanelType::Top => AlignItems::Start,
                    UiPanelType::Right | UiPanelType::Bottom => AlignItems::End,
                },
                justify_content: match panel_type {
                    UiPanelType::Left | UiPanelType::Top => JustifyContent::Start,
                    UiPanelType::Right | UiPanelType::Bottom => JustifyContent::End,
                },
                flex_direction: match panel_type {
                    UiPanelType::Left | UiPanelType::Right => FlexDirection::Column,
                    UiPanelType::Top | UiPanelType::Bottom => FlexDirection::Row,
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                // Window Node
                .spawn((
                    id,
                    UiWindowPanel(panel_type),
                    Node {
                        width: match panel_type {
                            UiPanelType::Left | UiPanelType::Right => size,
                            UiPanelType::Top | UiPanelType::Bottom => percent(100),
                        },
                        height: match panel_type {
                            UiPanelType::Left | UiPanelType::Right => percent(100),
                            UiPanelType::Top | UiPanelType::Bottom => size,
                        },
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        flex_direction: match panel_type {
                            UiPanelType::Left | UiPanelType::Right => FlexDirection::Column,
                            UiPanelType::Top | UiPanelType::Bottom => FlexDirection::Row,
                        },
                        row_gap: match panel_type {
                            UiPanelType::Left | UiPanelType::Right => px(0),
                            UiPanelType::Top | UiPanelType::Bottom => ui_theme.window.item_spacing,
                        },
                        column_gap: match panel_type {
                            UiPanelType::Left | UiPanelType::Right => ui_theme.window.item_spacing,
                            UiPanelType::Top | UiPanelType::Bottom => px(0),
                        },
                        padding: ui_theme.window.padding,
                        ..default()
                    },
                    BackgroundColor(ui_theme.window.colour),
                    BorderColor::all(ui_theme.window.border_colour),
                ))
                // Children from function
                .with_children(children);
        })
        .id()
}

pub fn spawn_dialog(
    id: UiWindowId,
    ui_theme: &UiTheme,
    commands: &mut Commands,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    id,
                    UiWindowDialog,
                    Node {
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
                .with_children(children);
        })
        .id()
}

pub fn spawn_floating(
    id: UiWindowId,
    aligning_node: Node,
    ui_theme: &UiTheme,
    commands: &mut Commands,
    children: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
) -> Entity {
    // Make the position relative to the centre
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..aligning_node
        })
        .with_children(|parent| {
            parent
                .spawn((
                    id,
                    UiWindowFloating,
                    Node {
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
                .with_children(children);
        })
        .id()
}
