use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::{cell_editor::state::CellEditorState, ui::ui_widget::UiElement};

use super::{UiTheme, ui_widget::ButtonType};

pub fn spawn_ui_element(parent: &mut RelatedSpawnerCommands<ChildOf>, label: &str, element: UiElement, ui_theme: &UiTheme) {
    parent.spawn(match element {
        UiElement::Button(_button_type) => {
            (
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
                element,
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
            )
        }
    });
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn ui_button_update(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, &UiElement),
        Changed<Interaction>,
    >,
    mut editor_state: ResMut<CellEditorState>,
) {
    for (entity, interaction, mut colour, mut border_colour, element) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                match element {
                    UiElement::Button(button_type) => {
                        *colour = ui_theme.button.pressed_colour.into();
                        *border_colour = ui_theme.button.border_pressed_colour.into();

                        // TODO Run the functions for each button
                        match button_type {
                            ButtonType::Save => {
                                editor_state.dialogs.open_save_dialog();
                            }
                            ButtonType::Load => todo!(),
                        }
                    }
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                match element {
                    UiElement::Button(_button_type) => {
                        *colour = ui_theme.button.hover_colour.into();
                        *border_colour = ui_theme.button.border_hover_colour.into();
                    }
                }
            }
            Interaction::None => {
                input_focus.clear();

                match element {
                    UiElement::Button(_button_type) => {
                        *colour = ui_theme.button.normal_colour.into();
                        *border_colour = ui_theme.button.border_colour.into();
                    }
                }
            }
        }
    }
}
