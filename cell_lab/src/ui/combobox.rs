use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::ui::UiTheme;

#[derive(Component, Debug, Copy, Clone)]
pub enum ComboboxId {
    SplitType,
}

#[derive(Component)]
pub struct Combobox {
    pub options: Vec<String>,
    pub selected: usize,
}

#[derive(Component)]
pub struct ComboboxOption {
    pub index: usize,
}

#[derive(Component)]
pub struct ComboboxSelectOption;

#[derive(Component)]
pub struct ComboboxOptionText;

#[derive(Component)]
pub struct ComboboxSelectOptionText;

#[allow(clippy::too_many_lines)]
pub fn spawn_combobox<S: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    combobox_id: ComboboxId,
    label: S,
    options: &[S],
    ui_theme: &UiTheme,
) {
    // Ensure that the options Vec has at least one option
    if options.is_empty() {
        eprintln!("Combobox options was an empty Vec");
        return;
    }

    let options: Vec<_> = options.iter().map(AsRef::as_ref).map(ToString::to_string).collect();

    let selected_option = options[0].clone();

    // Create the children bundles for each option
    let children: Vec<_> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            (
                Node {
                    padding: ui_theme.combobox.option_padding,
                    border: ui_theme.border,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: ui_theme.border_radius,
                    display: Display::None,
                    ..default()
                },
                // Mark as a combobox option
                ComboboxOption { index: i },
                // Set the background and border colours
                BackgroundColor(if i == 0 {
                    ui_theme.combobox.normal_selected_colour
                } else {
                    ui_theme.combobox.normal_colour
                }),
                BorderColor::all(ui_theme.combobox.border_colour),
                // Add the interaction component
                Interaction::default(),
                // Add the text for the option
                children![(
                    Text::new(option),
                    TextFont {
                        font: ui_theme.font.clone(),
                        font_size: ui_theme.combobox.font_size,
                        ..default()
                    },
                    ui_theme.text_colour,
                    ui_theme.text_shadow,
                    ComboboxOptionText
                )],
            )
        })
        .collect();

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
            parent.spawn((
                Text::new(label.as_ref()),
                TextFont {
                    font: ui_theme.font.clone(),
                    font_size: ui_theme.label_font_size,
                    ..default()
                },
                ui_theme.text_colour,
                ui_theme.text_shadow,
            ));

            parent
                .spawn((
                    // Create a commbobox root node
                    Node {
                        padding: ui_theme.combobox.padding,
                        border: ui_theme.border,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        border_radius: ui_theme.border_radius,
                        row_gap: ui_theme.combobox.option_spacing,
                        ..default()
                    },
                    // Make it a combobox
                    Combobox { options, selected: 0 },
                    // Mark with ID
                    combobox_id,
                    // Set the colours
                    BorderColor::all(ui_theme.combobox.border_colour),
                    BackgroundColor(ui_theme.combobox.normal_colour),
                ))
                .with_child((
                    Node {
                        padding: ui_theme.combobox.option_padding,
                        border: ui_theme.border,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: ui_theme.border_radius,
                        ..default()
                    },
                    // Mark as a combobox selected option
                    ComboboxSelectOption,
                    // Set the background and border colours
                    BackgroundColor(ui_theme.combobox.normal_valuebox_colour),
                    BorderColor::all(ui_theme.combobox.border_colour),
                    // Add the interaction component
                    Interaction::default(),
                    // Add the text for the option
                    children![(
                        Text::new(selected_option),
                        TextFont {
                            font: ui_theme.font.clone(),
                            font_size: ui_theme.combobox.font_size,
                            ..default()
                        },
                        ui_theme.text_colour,
                        ui_theme.text_shadow,
                        ComboboxSelectOptionText
                    )],
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
pub fn combobox_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut combobox_query: Query<
        (&Node, &ComboboxId, &mut Combobox, &Children),
        (
            Without<ComboboxOption>,
            Without<ComboboxSelectOption>,
            Without<ComboboxSelectOptionText>,
        ),
    >,
    mut combobox_selected_option_queries: ParamSet<(
        Query<
            (&Interaction, &mut BackgroundColor, &mut BorderColor, &ChildOf, &Children),
            (
                Changed<Interaction>,
                With<ComboboxSelectOption>,
                Without<ComboboxOption>,
                Without<ComboboxSelectOptionText>,
            ),
        >,
        Query<
            &Children,
            (
                With<ComboboxSelectOption>,
                Without<ComboboxOption>,
                Without<ComboboxSelectOptionText>,
            ),
        >,
    )>,
    mut combobox_selected_option_text_query: Query<
        &mut Text,
        (
            With<ComboboxSelectOptionText>,
            Without<ComboboxSelectOption>,
            Without<ComboboxOption>,
        ),
    >,
    mut combobox_options_queries: ParamSet<(
        Query<
            (
                Entity,
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &ComboboxOption,
                &ChildOf,
            ),
            (
                Changed<Interaction>,
                With<ComboboxOption>,
                Without<ComboboxSelectOption>,
                Without<ComboboxSelectOptionText>,
            ),
        >,
        Query<
            (&mut Node, &mut BackgroundColor, &mut BorderColor),
            (
                With<ComboboxOption>,
                Without<ComboboxSelectOption>,
                Without<ComboboxSelectOptionText>,
            ),
        >,
    )>,
) {
    // Check interactions with the select option

    for (interaction, mut colour, mut border_colour, parent, _) in &mut combobox_selected_option_queries.p0() {
        match *interaction {
            Interaction::Pressed => {
                // Change the colour
                colour.0 = ui_theme.combobox.pressed_value_box_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_pressed_colour);

                // Check the other children of the combobox to make them visible
                if let Ok((parent_node, _, _, children)) = combobox_query.get_mut(parent.parent()) {
                    for &child in children {
                        if let Ok((mut node, _, _)) = combobox_options_queries.p1().get_mut(child) {
                            node.display = parent_node.display;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // Change the colour
                colour.0 = ui_theme.combobox.hovered_value_box_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_hovered_colour);
            }
            Interaction::None => {
                // Change the colour
                colour.0 = ui_theme.combobox.normal_valuebox_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_pressed_colour);
            }
        }
    }

    let mut siblings_to_make_invisible = Vec::new();
    let mut selected_string = None;

    for (entity, interaction, mut colour, mut border_colour, combobox_option, parent) in &mut combobox_options_queries.p0() {
        // Get the parent of the combobox option, and get its components
        if let Ok((_parent_node, combobox_id, mut combobox, parent_children)) = combobox_query.get_mut(parent.parent()) {
            match *interaction {
                Interaction::Pressed => {
                    input_focus.set(entity);

                    // Select this combobox option
                    combobox.selected = combobox_option.index;
                    selected_string = Some(combobox.options[combobox.selected].clone());

                    // Change the colour
                    colour.0 = ui_theme.combobox.pressed_selected_colour;
                    *border_colour = BorderColor::all(ui_theme.combobox.border_pressed_colour);

                    // Set combobox children display to None of the parent combobox entity
                    for child in parent_children {
                        siblings_to_make_invisible.push(*child);
                    }

                    // TODO Do a function based on combobox ID
                    match combobox_id {
                        ComboboxId::SplitType => {}
                    }
                }
                Interaction::Hovered => {
                    input_focus.set(entity);

                    // Change the colour, depending on selection
                    if combobox.selected == combobox_option.index {
                        colour.0 = ui_theme.combobox.hovered_selected_colour;
                    } else {
                        colour.0 = ui_theme.combobox.hovered_colour;
                    }

                    *border_colour = BorderColor::all(ui_theme.combobox.border_hovered_colour);
                }
                Interaction::None => {
                    input_focus.clear();

                    // Change the colour, depending on selection
                    if combobox.selected == combobox_option.index {
                        colour.0 = ui_theme.combobox.normal_selected_colour;
                    } else {
                        colour.0 = ui_theme.combobox.normal_colour;
                    }

                    *border_colour = BorderColor::all(ui_theme.combobox.border_colour);
                }
            }
        }
    }

    // Set siblings' display to None
    for sibling in siblings_to_make_invisible {
        // Get the sibling's components
        if let Ok((mut sibling_node, mut sibling_colour, mut sibling_border_colour)) =
            combobox_options_queries.p1().get_mut(sibling)
        {
            // Set the colours to show it is no longer deselected
            sibling_colour.0 = ui_theme.combobox.normal_colour;
            *sibling_border_colour = BorderColor::all(ui_theme.combobox.border_colour);

            sibling_node.display = Display::None;
        }
    }

    // Change the text in the value box for commbobox
    for children in &mut combobox_selected_option_queries.p1() {
        for &child in children {
            // If this child is the select option's text, and selected string was set
            if let Ok(mut text) = combobox_selected_option_text_query.get_mut(child)
                && let Some(selected_string) = &selected_string
            {
                **text = selected_string.clone();
            }
        }
    }
}
