use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::ui::{
    ColourPickerEvent, ColourPickerMaterial, SliderHueMaterial, SliderId, UiTheme, rgb_to_hsv, spawn_slider_with_material,
    spawn_vertical,
};

#[derive(Component, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ColourPickerId {
    SelectedCellColour,
}

#[derive(Component)]
pub struct ColourPicker {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl ColourPicker {
    #[must_use]
    pub const fn to_hsv(&self) -> (f32, f32, f32) {
        (self.hue, self.saturation, self.value)
    }

    #[must_use]
    pub const fn to_colour(&self) -> Color {
        let (h, s, v) = self.to_hsv();
        Color::hsv(h, s, v)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_colour_picker(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    initial_value: Color,
    colour_picker_id: ColourPickerId,
    ui_theme: &UiTheme,

    window_scale: f32,
    window_width: f32,

    colour_picker_materials: &mut Assets<ColourPickerMaterial>,
    slider_hue_materials: &mut Assets<SliderHueMaterial>,
) -> Option<Entity> {
    let (hue, saturation, value) = rgb_to_hsv(initial_value);

    spawn_vertical(parent, ui_theme, |parent| {
        let colour_picker_entity = parent
            .spawn((
                // Create a colour picker shape
                Node {
                    width: ui_theme.colour_picker.expanded_size,
                    height: ui_theme.colour_picker.expanded_size,
                    padding: ui_theme.colour_picker.padding,
                    border: ui_theme.border,
                    ..default()
                },
                // Mark it as a colour picker
                ColourPicker { hue, saturation, value },
                // Mark with ID
                colour_picker_id,
                // Make it interactable
                Interaction::default(),
                // Set the colours
                BorderColor::all(ui_theme.colour_picker.border_colour),
                // Add the UiMaterial
                MaterialNode(colour_picker_materials.add(ColourPickerMaterial {
                    hue,
                    selected_uv: Vec2::new(saturation, 1.0 - value),
                })),
            ))
            .id();

        spawn_slider_with_material(
            parent,
            Some(colour_picker_entity),
            SliderId::ColourPickerHue,
            "",
            hue,
            0.0..=360.0,
            window_scale,
            window_width,
            slider_hue_materials,
            ui_theme,
        );

        Some(colour_picker_entity)
    })
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn colour_picker_interaction_system(
    windows: Query<&Window>,
    mut interaction_query: Query<(
        &Node,
        &UiGlobalTransform,
        &Interaction,
        &ColourPickerId,
        &mut ColourPicker,
        &MaterialNode<ColourPickerMaterial>,
    )>,
    mut colour_picker_event_writer: MessageWriter<ColourPickerEvent>,
    mut ui_materials: ResMut<Assets<ColourPickerMaterial>>,
) {
    // Get window and its properties
    let Ok(window) = windows.single() else { return };
    let scale = window.scale_factor();
    let win_size = window.width();

    // Get the cursor position
    let Some(cursor_pos) = window.cursor_position() else { return };

    for (node, transform, interaction, colour_picker_id, mut colour_picker, material_handle) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed | Interaction::Hovered => {
                let Ok(node_size) = node.width.resolve(scale, win_size, Vec2::splat(win_size)) else {
                    eprintln!("Could not calculate colour picker width");
                    continue;
                };

                // Calculate the extents of the colour picker
                let centre = transform.translation.xy();
                let min = centre - node_size * 0.5;
                let max = centre + node_size * 0.5;

                // Don't do anything if the cursor isn't within the colour picker area
                if cursor_pos.x < min.x || cursor_pos.x > max.x || cursor_pos.y < min.y || cursor_pos.y > max.y {
                    continue;
                }

                let (u, v) = ((cursor_pos.x - min.x) / node_size, (cursor_pos.y - min.y) / node_size);

                if *interaction == Interaction::Pressed {
                    // Set the saturation and value based on cursor position
                    colour_picker.saturation = u.clamp(0.0, 1.0);
                    colour_picker.value = (1.0 - v).clamp(0.0, 1.0);

                    // Set the cursor position within the colour picker material
                    if let Some(material) = ui_materials.get_mut(material_handle) {
                        material.selected_uv = Vec2::new(colour_picker.saturation, 1.0 - colour_picker.value);
                    }

                    // Fire an event to trigger this colour_picker
                    colour_picker_event_writer.write(ColourPickerEvent {
                        id: *colour_picker_id,
                        new_value: colour_picker.to_colour(),
                    });
                }
            }
            Interaction::None => {}
        }
    }
}
