use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::ui::UiTheme;

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

pub fn spawn_radio<S: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    radio_id: RadioId,
    options: &[S],
    ui_theme: &UiTheme,
) {
    // Ensure that the options Vec has at least one option
    if options.is_empty() {
        eprintln!("Radio options was an empty Vec");
        return;
    }

    let options: Vec<_> = options.iter().map(AsRef::as_ref).map(ToString::to_string).collect();

    // Create the children bundles for each option
    let children: Vec<_> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            (
                Node {
                    padding: ui_theme.radio.option_padding,
                    border: ui_theme.border,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: ui_theme.border_radius,
                    ..default()
                },
                // Mark as a radio option
                RadioOption { index: i },
                // Set the background and border colours
                BackgroundColor(if i == 0 {
                    ui_theme.radio.normal_selected_colour
                } else {
                    ui_theme.radio.normal_colour
                }),
                BorderColor::all(ui_theme.radio.border_colour),
                // Add the interaction component
                Interaction::default(),
                // Add the text for the option
                children![(
                    Text::new(option),
                    TextFont {
                        font: ui_theme.font.clone(),
                        font_size: ui_theme.radio.font_size,
                        ..default()
                    },
                    ui_theme.text_colour,
                    ui_theme.text_shadow,
                    RadioOptionText
                )],
            )
        })
        .collect();

    parent
        .spawn((
            // Create a radio root node
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
            // Make it a radio
            Radio { options, selected: 0 },
            // Mark with ID
            radio_id,
            // Set the colours
            BorderColor::all(ui_theme.radio.border_colour),
            BackgroundColor(ui_theme.radio.normal_colour),
        ))
        // Add the options
        .with_children(|parent| {
            for child in children {
                parent.spawn(child);
            }
        });
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn radio_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut radio_query: Query<(&RadioId, &mut Radio, &Children), Without<RadioOption>>,
    mut radio_options_queries: ParamSet<(
        Query<
            (
                Entity,
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &RadioOption,
                &ChildOf,
            ),
            Changed<Interaction>,
        >,
        Query<(&mut BackgroundColor, &mut BorderColor, &RadioOption)>,
    )>,
) {
    let mut siblings_to_deselect = Vec::new();

    for (entity, interaction, mut colour, mut border_colour, radio_option, parent) in &mut radio_options_queries.p0() {
        // Get the parent of the radio option, and get its components
        if let Ok((radio_id, mut radio, parent_children)) = radio_query.get_mut(parent.parent()) {
            match *interaction {
                Interaction::Pressed => {
                    input_focus.set(entity);

                    // Select this radio option
                    radio.selected = radio_option.index;

                    // Change the colour
                    colour.0 = ui_theme.radio.pressed_selected_colour;
                    *border_colour = BorderColor::all(ui_theme.radio.border_pressed_colour);

                    // Deselect other children of the parent radio entity
                    for child in parent_children {
                        siblings_to_deselect.push((*child, radio_option.index));
                    }

                    // TODO Do a function based on radio ID
                    match radio_id {
                        RadioId::SplitType => {}
                    }
                }
                Interaction::Hovered => {
                    input_focus.set(entity);

                    // Change the colour, depending on selection
                    if radio.selected == radio_option.index {
                        colour.0 = ui_theme.radio.hovered_selected_colour;
                    } else {
                        colour.0 = ui_theme.radio.hovered_colour;
                    }

                    *border_colour = BorderColor::all(ui_theme.radio.border_hovered_colour);
                }
                Interaction::None => {
                    input_focus.clear();

                    // Change the colour, depending on selection
                    if radio.selected == radio_option.index {
                        colour.0 = ui_theme.radio.normal_selected_colour;
                    } else {
                        colour.0 = ui_theme.radio.normal_colour;
                    }

                    *border_colour = BorderColor::all(ui_theme.radio.border_colour);
                }
            }
        }
    }

    // Deselect siblings
    for (sibling, selected_index) in siblings_to_deselect {
        // Get the sibling's components
        if let Ok((mut sibling_colour, mut sibling_border_colour, sibling_radio_option)) =
            radio_options_queries.p1().get_mut(sibling)
        {
            // This sibling is not actually the radio option that was just selected
            if sibling_radio_option.index != selected_index {
                // Set the colours to show it is no longer deselected
                sibling_colour.0 = ui_theme.radio.normal_colour;
                *sibling_border_colour = BorderColor::all(ui_theme.radio.border_colour);
            }
        }
    }
}
