use std::ops::RangeInclusive;

use bevy::{ecs::relationship::RelatedSpawnerCommands, input_focus::InputFocus, prelude::*};

use crate::ui::{SliderEvent, UiTheme, spawn_horizontal, spawn_label};

#[derive(Component, Debug, Copy, Clone)]
pub enum SliderId {
    SplitEnergy,
    SplitAge,
    SplitFraction,
    SplitAngle,
    SplitForce,
    Daughter1Angle,
    Daughter2Angle,
    CellEditorAge,
}

#[derive(Component)]
pub struct Slider {
    pub percent: f32, // 0 --> 1
    pub range: RangeInclusive<f32>,
}

impl Slider {
    #[must_use]
    pub fn get_value(&self) -> f32 {
        self.percent * (self.range.end() - self.range.start()) + self.range.start()
    }
}

#[derive(Component)]
pub struct SliderHandle;

#[derive(Component)]
pub struct ActiveSlider;

pub fn spawn_slider<S: AsRef<str>>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    slider_id: SliderId,
    label: S,
    initial_value: f32,
    range: RangeInclusive<f32>,
    ui_theme: &UiTheme,
) -> Option<Entity> {
    // Ensure that inital value is within the range
    if !range.contains(&initial_value) {
        eprintln!("Slider initial value was outside of values range: {slider_id:?}");
        return None;
    }

    let percent = (initial_value - range.start()) / (range.end() - range.start());

    spawn_horizontal(parent, ui_theme, |parent| {
        // Add a label for the ui element
        spawn_label(parent, label, ui_theme);

        Some(
            parent
                .spawn((
                    // Create a slider shape
                    Node {
                        padding: ui_theme.slider.padding,
                        border: ui_theme.border,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: ui_theme.border_radius,
                        width: ui_theme.slider.width,
                        height: ui_theme.slider.height,
                        ..default()
                    },
                    // Mark as a slider
                    Slider { percent, range },
                    // Mark with ID
                    slider_id,
                    // Set the colours
                    BorderColor::all(ui_theme.slider.track_border_colour),
                    BackgroundColor(ui_theme.slider.track_colour),
                    // Add the interaction component
                    Interaction::default(),
                    // Add the text
                    children![(
                        Node {
                            width: ui_theme.slider.handle_width,
                            height: ui_theme.slider.handle_height,
                            position_type: PositionType::Absolute,
                            // TODO Need to account for handle width
                            left: Val::Percent(percent * 100.),
                            border_radius: ui_theme.border_radius,
                            ..default()
                        },
                        BackgroundColor(ui_theme.slider.handle_colour),
                        SliderHandle
                    )],
                ))
                .id(),
        )
    })
}

#[allow(clippy::type_complexity)]
pub fn slider_begin_drag_system(
    mut commands: Commands,
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Slider>)>,
) {
    for (entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            commands.entity(entity).insert(ActiveSlider);
        }
    }
}

pub fn slider_drag_system(
    windows: Query<&Window>,
    mut sliders: Query<(&mut Slider, &SliderId, &Node, &UiGlobalTransform, &Children), With<ActiveSlider>>,
    mut handles: Query<&mut Node, (With<SliderHandle>, Without<Slider>)>,
    mut slider_event_writer: MessageWriter<SliderEvent>,
) {
    // Get window properties to calculate the pixel size of different elements
    let Ok(window) = windows.single() else { return };
    let scale = window.scale_factor();
    let win_width = window.width();

    // Get the mouse position
    if let Some(cursor_pos) = window.cursor_position() {
        // Iterate over active sliders (should only be one) to adjust handle position
        for (mut slider, slider_id, node, transform, children) in &mut sliders {
            // Find the slider handle for this slider
            for &child in children {
                // Get the node for the slider handle
                if let Ok(mut handle_node) = handles.get_mut(child) {
                    // Get the width of the track
                    let Ok(track_width) = node.width.resolve(scale, win_width, Vec2::splat(win_width)) else {
                        continue;
                    };
                    // Get the width of the handle
                    let Ok(handle_width) = handle_node.width.resolve(scale, track_width, Vec2::splat(track_width)) else {
                        continue;
                    };

                    // Get the width the horizontal borders added together
                    let border_width = {
                        // Get the left border size
                        let Ok(border_left) = node.border.left.resolve(scale, track_width, Vec2::splat(track_width)) else {
                            continue;
                        };

                        // Get the right border size
                        let Ok(border_right) = node.border.right.resolve(scale, track_width, Vec2::splat(track_width)) else {
                            continue;
                        };

                        // Calculate total border size (Horizontal)
                        border_left + border_right
                    };

                    // Adjust the track width to account for handle width and border width
                    let adjusted_track_width = track_width - border_width - handle_width;

                    // Scale the cursor position to bring it between 0 and 1 when on the slider
                    let scaled_x = (cursor_pos.x - transform.translation.x) / adjusted_track_width + 0.5;
                    slider.percent = scaled_x.clamp(0.0, 1.0);

                    handle_node.left = Val::Px(slider.percent * adjusted_track_width);

                    // Send an event for this slider drag
                    slider_event_writer.write(SliderEvent {
                        id: *slider_id,
                        new_value: slider.get_value(),
                    });

                    // There is only one handle for the slider
                    break;
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn slider_release_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<ActiveSlider>>,
) {
    if mouse.just_released(MouseButton::Left) {
        for entity in &query {
            commands.entity(entity).remove::<ActiveSlider>();
        }
    }
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn slider_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    ui_theme: Res<UiTheme>,
    mut interaction_query: Query<(Entity, &Interaction, &Children), (Changed<Interaction>, With<Slider>)>,
    mut handles_query: Query<(Entity, &mut BackgroundColor, &mut BorderColor), With<SliderHandle>>,
) {
    for (entity, interaction, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);

                // Get the slider handle child
                for child in children {
                    // If this child is the slider handle, change its colour
                    if let Ok((_handle_entity, mut handle_bg_colour, mut handle_border_colour)) = handles_query.get_mut(*child) {
                        handle_bg_colour.0 = ui_theme.slider.handle_pressed_colour;
                        *handle_border_colour = BorderColor::all(ui_theme.slider.handle_pressed_border_colour);

                        // There should only be one handle
                        break;
                    }
                }
            }
            Interaction::Hovered => {
                input_focus.set(entity);

                // Get the slider handle child
                for child in children {
                    // If this child is the slider handle, change its colour
                    if let Ok((_handle_entity, mut handle_bg_colour, mut handle_border_colour)) = handles_query.get_mut(*child) {
                        handle_bg_colour.0 = ui_theme.slider.handle_hovered_colour;
                        *handle_border_colour = BorderColor::all(ui_theme.slider.handle_hovered_border_colour);

                        // There should only be one handle
                        break;
                    }
                }
            }
            Interaction::None => {
                input_focus.clear();

                // Get the slider handle child
                for child in children {
                    // If this child is the slider handle, change its colour
                    if let Ok((_handle_entity, mut handle_bg_colour, mut handle_border_colour)) = handles_query.get_mut(*child) {
                        handle_bg_colour.0 = ui_theme.slider.handle_colour;
                        *handle_border_colour = BorderColor::all(ui_theme.slider.handle_border_colour);

                        // There should only be one handle
                        break;
                    }
                }
            }
        }
    }
}
