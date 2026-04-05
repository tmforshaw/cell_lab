use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::{UiElement, UiTheme};

#[derive(Component, Debug, Copy, Clone)]
pub enum ButtonId {
    Save,
    Load,
}

pub fn spawn_button(parent: &mut RelatedSpawnerCommands<ChildOf>, label: &str, button_id: ButtonId, ui_theme: &UiTheme) {
    parent.spawn((
        // Create a button shape
        Node {
            padding: ui_theme.button.padding,
            border: ui_theme.border,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: ui_theme.border_radius,
            ..default()
        },
        // Make it clickable
        Button,
        // Mark UiElement type
        UiElement::Button(button_id),
        // Set the colours
        BorderColor::all(ui_theme.button.border_colour),
        BackgroundColor(ui_theme.button.normal_colour),
        // Add the text
        children![(
            Text::new(label),
            TextFont {
                font: ui_theme.font.clone(),
                font_size: 32.0,
                ..default()
            },
            ui_theme.text_colour,
            ui_theme.text_shadow,
        )],
    ));
}
