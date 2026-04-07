use bevy::{
    ecs::relationship::RelatedSpawnerCommands,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    input_focus::InputFocus,
    prelude::*,
};

use crate::ui::{TextInputEvent, UiTheme, spawn_horizontal, spawn_label};

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextInputId {
    SaveFilename,
}

#[derive(Component)]
pub struct TextInput {
    pub value: String,
    pub filter_map: Option<fn(&char) -> Option<char>>,
}

#[derive(Component)]
pub struct TextInputField;

pub fn spawn_text_input<S1: AsRef<str>, S2: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    input_id: TextInputId,
    label: S1,
    initial_value: S2,
    filter_map: Option<fn(&char) -> Option<char>>,
    ui_theme: &UiTheme,
) {
    spawn_horizontal(parent, ui_theme, |parent| {
        spawn_label(parent, label.as_ref(), ui_theme);

        parent
            .spawn(Node {
                min_width: Val::Px(ui_theme.text_input.min_width),
                max_width: Val::Px(ui_theme.text_input.max_width),
                flex_grow: 0.,
                ..default()
            })
            .with_child((
                Node {
                    min_width: percent(100),
                    padding: ui_theme.text_input.padding,
                    border: ui_theme.border,
                    border_radius: ui_theme.border_radius,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                TextInput {
                    value: initial_value.as_ref().to_string(),
                    filter_map,
                },
                input_id,
                Interaction::default(),
                BackgroundColor(ui_theme.text_input.normal_colour),
                BorderColor::all(ui_theme.text_input.border_colour),
                children![(
                    Text::new(initial_value.as_ref()),
                    TextFont {
                        font: ui_theme.font.clone(),
                        font_size: ui_theme.inner_font_size,
                        ..default()
                    },
                    ui_theme.text_colour,
                    ui_theme.text_shadow,
                    TextInputField
                )],
            ));
    });
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn text_input_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut query: Query<(Entity, &Interaction, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
) {
    for (entity, interaction, mut colour, mut border) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                // Change the colour depending on interaction type
                colour.0 = ui_theme.text_input.pressed_colour;
                *border = BorderColor::all(ui_theme.text_input.border_pressed_colour);
            }
            Interaction::Hovered => {
                // Change the colour depending on interaction type
                colour.0 = ui_theme.text_input.hovered_colour;
                *border = BorderColor::all(ui_theme.text_input.border_hovered_colour);
            }
            Interaction::None => {
                // Change the colour depending on interaction type
                if input_focus.0 == Some(entity) {
                    colour.0 = ui_theme.text_input.focused_colour;
                } else {
                    colour.0 = ui_theme.text_input.normal_colour;
                }

                *border = BorderColor::all(ui_theme.text_input.border_colour);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn text_input_typing_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut keyboard_reader: MessageReader<KeyboardInput>,
    mut query: Query<(Entity, &TextInputId, &mut TextInput, &Children)>,
    mut text_query: Query<&mut Text, (With<TextInputField>, Without<TextInput>)>,
    mut text_input_event_writer: MessageWriter<TextInputEvent>,
) {
    for (entity, input_id, mut text_input, children) in &mut query {
        // Only process keyboard input if focused
        if input_focus.0 == Some(entity) {
            for input in keyboard_reader.read() {
                if input.state == ButtonState::Pressed {
                    let font_size_width = ui_theme.inner_font_size / 1.5;
                    let text_width = (text_input.value.chars().count()) as f32 * font_size_width;

                    match input.logical_key.clone() {
                        // Submit the text from TextInput
                        Key::Enter => {
                            // Send a TextInputEvent
                            text_input_event_writer.write(TextInputEvent {
                                id: *input_id,
                                new_value: text_input.value.clone(),
                            });

                            // Clear the TextInput
                            text_input.value.clear();

                            // Clear the input focus
                            input_focus.clear();
                        }
                        // Remove text in TextInput
                        Key::Backspace => {
                            text_input.value.pop();
                        }
                        // Add a space to TextInput
                        Key::Space => {
                            // Only add character if it will not be longer than max width
                            if text_width + font_size_width <= ui_theme.text_input.max_width {
                                text_input.value.push(' ');
                            }
                        }
                        Key::Character(chars) => {
                            // Filter out the control characters, and apply the filter map specified in TextInput (if it exists)
                            let filtered_chars = chars
                                .chars()
                                .filter(|c| !c.is_control())
                                .filter_map(|c| text_input.filter_map.map_or(Some(c), |input_filter_map| input_filter_map(&c))) // Apply the filter map given by TextInput
                                .collect::<String>();

                            let added_text_width = filtered_chars.len() as f32 * font_size_width;

                            // The new text doesn't overflow the maximum size
                            if text_width + added_text_width <= ui_theme.text_input.max_width {
                                text_input.value.push_str(filtered_chars.as_str());
                            } else {
                                // Text needs to be clipped
                                let char_space_left =
                                    ((ui_theme.text_input.max_width - text_width) / font_size_width).floor() as usize;

                                // Add only that amount of characters to the text_input value
                                text_input
                                    .value
                                    .push_str(filtered_chars.chars().take(char_space_left).collect::<String>().as_str());
                            }
                        }
                        _ => {}
                    }
                }
            }

            // Find the text child, and modify its value to the new value
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = text_input.value.clone();
                }
            }
        }
    }
}
