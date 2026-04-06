use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::ui::{CheckboxEvent, UiTheme, spawn_label};

#[derive(Component, Debug, Copy, Clone)]
pub enum CheckboxId {
    InitialMode,
}

#[derive(Component)]
pub struct Checkbox {
    pub selected: bool,
}

pub fn spawn_checkbox<S: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    checkbox_id: CheckboxId,
    label: S,
    initial_value: bool,
    ui_theme: &UiTheme,
) {
    parent
        .spawn(
            // Create a horizontal flex box for the label and the ui element
            Node {
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: ui_theme.label_gap,
                ..default()
            },
        )
        .with_children(|parent| {
            // Add a label for the ui element
            spawn_label(parent, label, ui_theme);

            // Create a checkbox shape
            parent.spawn((
                Node {
                    padding: ui_theme.checkbox.padding,
                    border: ui_theme.border,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: ui_theme.border_radius,
                    ..default()
                },
                // Make it a checkbox
                Checkbox { selected: initial_value },
                // Mark with ID
                checkbox_id,
                // Add the interaction component
                Interaction::default(),
                // Set the colours
                if initial_value {
                    BackgroundColor(ui_theme.checkbox.normal_selected_colour)
                } else {
                    BackgroundColor(ui_theme.checkbox.normal_colour)
                },
                BorderColor::all(ui_theme.checkbox.border_colour),
            ));
        });
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn checkbox_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &CheckboxId,
            &mut Checkbox,
        ),
        Changed<Interaction>,
    >,
    mut checkbox_event_writer: MessageWriter<CheckboxEvent>,
) {
    for (entity, interaction, mut colour, mut border_colour, checkbox_id, mut checkbox) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                // Toggle the selection
                checkbox.selected ^= true;

                // Change the colour based on selection
                if checkbox.selected {
                    colour.0 = ui_theme.checkbox.pressed_selected_colour;
                } else {
                    colour.0 = ui_theme.checkbox.pressed_colour;
                }

                *border_colour = BorderColor::all(ui_theme.checkbox.border_pressed_colour);

                // Trigger an event for the checkbox change
                checkbox_event_writer.write(CheckboxEvent {
                    entity,
                    id: *checkbox_id,
                    new_value: checkbox.selected,
                });
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                // Change the colour based on selection
                if checkbox.selected {
                    colour.0 = ui_theme.checkbox.hovered_selected_colour;
                } else {
                    colour.0 = ui_theme.checkbox.hovered_colour;
                }

                *border_colour = BorderColor::all(ui_theme.checkbox.border_hovered_colour);
            }
            Interaction::None => {
                input_focus.clear();

                // Change the colour based on selection
                if checkbox.selected {
                    colour.0 = ui_theme.checkbox.normal_selected_colour;
                    *border_colour = BorderColor::all(ui_theme.checkbox.border_hovered_colour);
                } else {
                    colour.0 = ui_theme.checkbox.normal_colour;
                }

                *border_colour = BorderColor::all(ui_theme.checkbox.border_colour);
            }
        }
    }
}
