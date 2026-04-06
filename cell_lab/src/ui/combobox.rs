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
pub struct ComboboxOptionContainer;

#[derive(Component)]
pub struct ComboboxValueBox;

#[derive(Component)]
pub struct ComboboxOptionText;

#[derive(Component)]
pub struct ComboboxValueBoxText;

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
                    ComboboxValueBox,
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
                        ComboboxValueBoxText
                    )],
                ))
                // Add a node with the options as its children
                .with_children(|parent| {
                    parent
                        .spawn((
                            // Make a Node to contain all the options
                            Node {
                                padding: ui_theme.combobox.padding,
                                border: ui_theme.border,
                                border_radius: ui_theme.border_radius,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                top: percent(100), // Move it below the value node
                                flex_direction: FlexDirection::Column,
                                row_gap: ui_theme.combobox.option_spacing,
                                position_type: PositionType::Absolute, // Make the options overlap the other content
                                ..default()
                            },
                            ZIndex(100), // Ensure that this node is drawn above the other nodes
                            // Set the colours
                            BorderColor::all(ui_theme.combobox.border_colour),
                            BackgroundColor(ui_theme.combobox.normal_colour),
                            // Mark as the options container
                            ComboboxOptionContainer,
                        ))
                        // Add the options
                        .with_children(|parent| {
                            for child in children {
                                parent.spawn(child);
                            }
                        });
                });
        });
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn combobox_toggle_system(
    ui_theme: Res<UiTheme>,
    mut input_focus: ResMut<InputFocus>,
    mut valueboxes: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, &ChildOf),
        (Changed<Interaction>, With<ComboboxValueBox>),
    >,
    mut containers: Query<&mut Node, With<ComboboxOptionContainer>>,
    comboboxes: Query<(&Node, &Children), (With<Combobox>, Without<ComboboxOptionContainer>)>,
) {
    for (entity, interaction, mut colour, mut border_colour, parent) in &mut valueboxes {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                // Set the colours depending on the interaction type
                colour.0 = ui_theme.combobox.pressed_valuebox_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_pressed_colour);

                // Get combobox parent of value box
                if let Ok((combobox_node, combobox_children)) = comboboxes.get(parent.parent()) {
                    // Find container for this combobox
                    for &child in combobox_children {
                        // Toggle the display visibility
                        if let Ok(mut node) = containers.get_mut(child) {
                            node.display = if node.display == Display::None {
                                combobox_node.display
                            } else {
                                Display::None
                            };

                            // There is only one container for the combobox
                            break;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                // Set the colours depending on the interaction type
                colour.0 = ui_theme.combobox.hovered_valuebox_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_hovered_colour);
            }
            Interaction::None => {
                input_focus.clear();

                // Set the colours depending on the interaction type
                colour.0 = ui_theme.combobox.normal_valuebox_colour;
                *border_colour = BorderColor::all(ui_theme.combobox.border_colour);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn combobox_option_select_system(
    ui_theme: Res<UiTheme>,
    mut input_focus: ResMut<InputFocus>,
    mut options: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ComboboxOption,
            &ChildOf,
        ),
        (Changed<Interaction>, Without<ComboboxValueBox>),
    >,
    mut containers: Query<(&mut Node, &ChildOf), With<ComboboxOptionContainer>>,
    mut comboboxes: Query<&mut Combobox>,
) {
    for (entity, interaction, mut colour, mut border_colour, option, parent) in &mut options {
        // Get the container for the option
        if let Ok((mut container_node, container_parent)) = containers.get_mut(parent.parent()) {
            // Get the combobox for the container
            if let Ok(mut combobox) = comboboxes.get_mut(container_parent.parent()) {
                match *interaction {
                    Interaction::Pressed => {
                        input_focus.set(entity);

                        // Set this option as the selected option
                        combobox.selected = option.index;

                        // No point in setting the colour since the options are going to be set to Display::none

                        // Close the options container
                        container_node.display = Display::None;
                    }
                    Interaction::Hovered => {
                        input_focus.set(entity);

                        if combobox.selected == option.index {
                            colour.0 = ui_theme.combobox.hovered_selected_colour;
                        } else {
                            colour.0 = ui_theme.combobox.hovered_colour;
                        }

                        *border_colour = BorderColor::all(ui_theme.combobox.border_hovered_colour);
                    }
                    Interaction::None => {
                        input_focus.clear();

                        if combobox.selected == option.index {
                            colour.0 = ui_theme.combobox.normal_selected_colour;
                        } else {
                            colour.0 = ui_theme.combobox.normal_colour;
                        }

                        *border_colour = BorderColor::all(ui_theme.combobox.border_colour);
                    }
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn combobox_text_update_system(
    ui_theme: Res<UiTheme>,
    comboboxes: Query<(&Combobox, &Children), Changed<Combobox>>,
    containers: Query<&Children, With<ComboboxOptionContainer>>,
    mut options: Query<(&mut BackgroundColor, &mut BorderColor, &ComboboxOption), Without<ComboboxValueBox>>,
    mut text_query: Query<&mut Text, With<ComboboxValueBoxText>>,
) {
    for (combobox, children) in &comboboxes {
        let selected = combobox.options[combobox.selected].clone();

        // Change the value box text to display the selected option
        for mut text in &mut text_query {
            **text = selected.clone();
        }

        // Get the container for the options
        for &child in children {
            if let Ok(container_children) = containers.get(child) {
                // Iterate over options in container
                for &child in container_children {
                    // If the child is an option
                    if let Ok((mut colour, mut border_colour, option)) = options.get_mut(child) {
                        // Set the option's colour depending on its selection status
                        if option.index == combobox.selected {
                            colour.0 = ui_theme.combobox.normal_selected_colour;
                        } else {
                            colour.0 = ui_theme.combobox.normal_colour;
                        }

                        *border_colour = BorderColor::all(ui_theme.combobox.border_colour);
                    }
                }
            }
        }
    }
}
