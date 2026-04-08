use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::{ui::ButtonEvent, ui::UiTheme};

#[derive(Component, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ButtonId {
    Save,
    Load,
    ReplaceModeWithDefault,
    ConfirmReplaceModeWithDefault,
    CloseAllDialogs,
    CloseOverwriteGenomeDialog,
    ConfirmOverwriteGenome,
    CloseSaveFilenameEmptyDialog,
    SubmitSaveFilename,
}

#[derive(Component, Debug)]
pub struct ButtonTarget(pub Option<Entity>);

pub fn spawn_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    target_entity: Option<Entity>,
    label: &str,
    button_id: ButtonId,
    ui_theme: &UiTheme,
) -> Option<Entity> {
    Some(
        parent
            .spawn((
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
                        font_size: ui_theme.inner_font_size,
                        ..default()
                    },
                    ui_theme.text_colour,
                    ui_theme.text_shadow,
                )],
            ))
            .insert_if(ButtonTarget(target_entity), || target_entity.is_some())
            .id(),
    )
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn button_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonId,
            Option<&ButtonTarget>,
        ),
        Changed<Interaction>,
    >,
    mut button_event_writer: MessageWriter<ButtonEvent>,
) {
    for (entity, interaction, mut colour, mut border_colour, button_id, button_target) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                // Change the colour depending on the interaction type
                colour.0 = ui_theme.button.pressed_colour;
                *border_colour = ui_theme.button.border_pressed_colour.into();

                // Fire an event to trigger this button
                button_event_writer.write(ButtonEvent {
                    target_entity: button_target.and_then(|target| target.0),
                    id: *button_id,
                });
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                // Change the colour depending on the interaction type
                colour.0 = ui_theme.button.hovered_colour;
                *border_colour = ui_theme.button.border_hovered_colour.into();
            }
            Interaction::None => {
                input_focus.clear();

                // Change the colour depending on the interaction type
                colour.0 = ui_theme.button.normal_colour;
                *border_colour = ui_theme.button.border_colour.into();
            }
        }
    }
}
