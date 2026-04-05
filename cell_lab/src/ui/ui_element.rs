use bevy::{input_focus::InputFocus, prelude::*};

use crate::cell_editor::state::CellEditorState;

use super::{ButtonId, SliderId, UiTheme};

#[derive(Component, Debug, Copy, Clone)]
pub enum UiElement {
    Button(ButtonId),
    Slider(SliderId),
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn ui_element_update(
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
                            ButtonId::Save => {
                                editor_state.dialogs.open_save_dialog();
                            }
                            ButtonId::Load => todo!(),
                        }
                    }
                    UiElement::Slider(_slider_id) => {}
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                match element {
                    UiElement::Button(_button_type) => {
                        *colour = ui_theme.button.hover_colour.into();
                        *border_colour = ui_theme.button.border_hover_colour.into();
                    }
                    UiElement::Slider(_slider_id) => {}
                }
            }
            Interaction::None => {
                input_focus.clear();

                match element {
                    UiElement::Button(_button_type) => {
                        *colour = ui_theme.button.normal_colour.into();
                        *border_colour = ui_theme.button.border_colour.into();
                    }
                    UiElement::Slider(_slider_id) => {}
                }
            }
        }
    }
}
