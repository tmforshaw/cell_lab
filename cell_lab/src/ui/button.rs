use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::{cell_editor::state::CellEditorState, ui::UiTheme};

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
        // Mark with ID
        button_id,
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

#[allow(clippy::needless_pass_by_value)]
pub fn button_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut interaction_query: Query<(Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, &ButtonId), Changed<Interaction>>,
    mut editor_state: ResMut<CellEditorState>,
) {
    for (entity, interaction, mut colour, mut border_colour, button_id) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                *colour = ui_theme.button.pressed_colour.into();
                *border_colour = ui_theme.button.border_pressed_colour.into();

                // TODO Run the functions for each button
                match button_id {
                    ButtonId::Save => {
                        editor_state.dialogs.open_save_dialog();
                    }
                    ButtonId::Load => todo!(),
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                *colour = ui_theme.button.hover_colour.into();
                *border_colour = ui_theme.button.border_hover_colour.into();
            }
            Interaction::None => {
                input_focus.clear();

                *colour = ui_theme.button.normal_colour.into();
                *border_colour = ui_theme.button.border_colour.into();
            }
        }
    }
}
