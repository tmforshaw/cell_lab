use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::ui::{RadioEvent, UiTheme, spawn_horizontal, spawn_label};

#[derive(Component, Debug, Copy, Clone)]
pub enum RadioId {
    SplitType,
}

#[derive(Component)]
pub struct Radio {
    pub options: Vec<String>,
    pub selected: usize,
}

#[derive(Component)]
pub struct RadioOption {
    pub index: usize,
}

#[derive(Component)]
pub struct RadioOptionText;

#[derive(Component, Copy, Clone)]
pub enum RadioStyle {
    Button,
    Text,
}

#[allow(clippy::too_many_lines)]
fn spawn_radio<S1: AsRef<str>, S2: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    radio_id: RadioId,
    radio_style: RadioStyle,
    label: S1,
    initial_selected: usize,
    options: &[S2],
    ui_theme: &UiTheme,
) {
    // Ensure that the options Vec has at least one option
    if options.is_empty() {
        eprintln!("Radio options was an empty Vec: {radio_id:?}");
        return;
    }

    // Ensure that initial selected is within the options length
    if initial_selected >= options.len() {
        eprintln!("Radio initial selected was outside of options Vec: {radio_id:?}");
        return;
    }

    let options: Vec<_> = options.iter().map(AsRef::as_ref).map(ToString::to_string).collect();

    // Create the children bundles for each option
    let children: Vec<_> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            (
                // Mark as a radio option
                RadioOption { index: i },
                match radio_style {
                    RadioStyle::Button => (
                        Node {
                            padding: ui_theme.radio.option_padding,
                            border: ui_theme.border,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: ui_theme.border_radius,
                            ..default()
                        },
                        // Set the background and border colours
                        BackgroundColor(if i == initial_selected {
                            ui_theme.radio.normal_selected_colour
                        } else {
                            ui_theme.radio.normal_colour
                        }),
                        BorderColor::all(ui_theme.radio.border_colour),
                    ),
                    RadioStyle::Text => (
                        Node {
                            padding: UiRect::all(px(0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::default(),
                    ),
                },
                // Add the interaction component
                Interaction::default(),
                // Add the text for the option
                children![(
                    Text::new(option),
                    TextFont {
                        font: ui_theme.font.clone(),
                        font_size: ui_theme.inner_font_size,
                        ..default()
                    },
                    ui_theme.text_colour,
                    ui_theme.text_shadow,
                    RadioOptionText
                )],
            )
        })
        .collect();

    spawn_horizontal(parent, ui_theme, |parent| {
        // Add a label for the ui element
        spawn_label(parent, label, ui_theme);

        parent
            .spawn((
                // Create a radio root node
                match radio_style {
                    RadioStyle::Button => (
                        Node {
                            padding: ui_theme.radio.padding,
                            border: ui_theme.border,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            border_radius: ui_theme.border_radius,
                            column_gap: ui_theme.radio.option_spacing,
                            ..default()
                        },
                        // Set the colours
                        BorderColor::all(ui_theme.radio.border_colour),
                        BackgroundColor(ui_theme.radio.normal_colour),
                    ),
                    RadioStyle::Text => (
                        Node {
                            padding: UiRect::all(px(0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: ui_theme.window.item_spacing,
                            ..default()
                        },
                        BorderColor::default(),
                        BackgroundColor(Color::NONE),
                    ),
                },
                // Make it a radio
                Radio {
                    options,
                    selected: initial_selected,
                },
                // Give the RadioStyle
                radio_style,
                // Mark with ID
                radio_id,
            ))
            // Add the options
            .with_children(|parent| {
                for child in children {
                    parent.spawn(child);
                }
            });
    });
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn radio_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut radio_query: Query<(&RadioId, &mut Radio, &RadioStyle, &Children), Without<RadioOption>>,
    mut radio_options_queries: ParamSet<(
        Query<
            (
                Entity,
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &RadioOption,
                &ChildOf,
                &Children,
            ),
            Changed<Interaction>,
        >,
        Query<(&mut BackgroundColor, &mut BorderColor, &RadioOption, &ChildOf, &Children)>,
    )>,
    mut radio_event_writer: MessageWriter<RadioEvent>,
    mut text_query: Query<&mut TextColor, With<RadioOptionText>>,
) {
    let mut siblings_to_deselect = Vec::new();

    for (entity, interaction, mut colour, mut border_colour, radio_option, parent, option_children) in
        &mut radio_options_queries.p0()
    {
        // Get the parent of the radio option, and get its components
        if let Ok((radio_id, mut radio, radio_style, parent_children)) = radio_query.get_mut(parent.parent()) {
            match *interaction {
                Interaction::Pressed => {
                    input_focus.set(entity);

                    // Select this radio option
                    radio.selected = radio_option.index;

                    match radio_style {
                        RadioStyle::Button => {
                            // Change the colour
                            colour.0 = ui_theme.radio.pressed_selected_colour;
                            *border_colour = BorderColor::all(ui_theme.radio.border_pressed_colour);
                        }
                        RadioStyle::Text => {
                            // Change text colour
                            for &child in option_children {
                                if let Ok(mut text_colour) = text_query.get_mut(child) {
                                    *text_colour = ui_theme.radio.text_selected_colour;

                                    // Should only be one text child
                                    break;
                                }
                            }
                        }
                    }

                    // Deselect other children of the parent radio entity
                    for child in parent_children {
                        siblings_to_deselect.push((*child, radio_option.index));
                    }

                    // Trigger an event for the radio value change
                    radio_event_writer.write(RadioEvent {
                        id: *radio_id,
                        new_value_index: radio.selected,
                    });
                }
                Interaction::Hovered => {
                    input_focus.set(entity);

                    // Change the colour depending on style
                    match radio_style {
                        RadioStyle::Button => {
                            // Change the colour, depending on selection
                            if radio.selected == radio_option.index {
                                colour.0 = ui_theme.radio.hovered_selected_colour;
                            } else {
                                colour.0 = ui_theme.radio.hovered_colour;
                            }

                            *border_colour = BorderColor::all(ui_theme.radio.border_hovered_colour);
                        }
                        RadioStyle::Text => {}
                    }
                }
                Interaction::None => {
                    input_focus.clear();

                    // Change the colour depending on style
                    match radio_style {
                        RadioStyle::Button => {
                            // Change the colour, depending on selection
                            if radio.selected == radio_option.index {
                                colour.0 = ui_theme.radio.normal_selected_colour;
                            } else {
                                colour.0 = ui_theme.radio.normal_colour;
                            }

                            *border_colour = BorderColor::all(ui_theme.radio.border_colour);
                        }
                        RadioStyle::Text => {}
                    }
                }
            }
        }
    }

    // Deselect siblings
    for (sibling, selected_index) in siblings_to_deselect {
        // Get the sibling's components
        if let Ok((mut sibling_colour, mut sibling_border_colour, sibling_radio_option, parent, option_children)) =
            radio_options_queries.p1().get_mut(sibling)
        {
            // Get the parent components (Radio Ui Element)
            if let Ok((_, _, radio_style, _)) = radio_query.get(parent.parent()) {
                // This sibling is not actually the radio option that was just selected
                if sibling_radio_option.index != selected_index {
                    match radio_style {
                        RadioStyle::Button => {
                            // Set the colours to show it is no longer deselected
                            sibling_colour.0 = ui_theme.radio.normal_colour;
                            *sibling_border_colour = BorderColor::all(ui_theme.radio.border_colour);
                        }
                        RadioStyle::Text => {
                            // Get the text for this option, and set it to the unselected colour
                            for &child in option_children {
                                if let Ok(mut text_colour) = text_query.get_mut(child) {
                                    *text_colour = ui_theme.text_colour;

                                    // Should only be one text child
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_radio_buttonlike<S1: AsRef<str>, S2: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    radio_id: RadioId,
    label: S1,
    initial_selected: usize,
    options: &[S2],
    ui_theme: &UiTheme,
) {
    spawn_radio(
        parent,
        radio_id,
        RadioStyle::Button,
        label,
        initial_selected,
        options,
        ui_theme,
    );
}

pub fn spawn_radio_textlike<S1: AsRef<str>, S2: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    radio_id: RadioId,
    label: S1,
    initial_selected: usize,
    options: &[S2],
    ui_theme: &UiTheme,
) {
    spawn_radio(parent, radio_id, RadioStyle::Text, label, initial_selected, options, ui_theme);
}
