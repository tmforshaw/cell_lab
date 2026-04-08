use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::GenomeBank,
    ui::{ColourPickerMaterial, SliderId, colour_picker::colour_picker_systems::ColourPicker},
};

#[derive(Message)]
pub struct SliderEvent {
    pub self_entity: Entity,
    pub target_entity: Option<Entity>,
    pub id: SliderId,
    pub new_value: f32,
}

pub fn slider_event_reader(
    mut events: MessageReader<SliderEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,

    mut colour_picker_query: Query<(&mut ColourPicker, &MaterialNode<ColourPickerMaterial>)>,
    mut ui_materials: ResMut<Assets<ColourPickerMaterial>>,
) {
    for ev in events.read() {
        match ev.id {
            SliderId::SplitEnergy => {
                // Set the split energy in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_energy = ev.new_value;
            }
            SliderId::SplitAge => {
                // Set the split age in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_age = ev.new_value;
            }
            SliderId::SplitFraction => {
                // Set the split fraction in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_fraction = ev.new_value;
            }
            SliderId::SplitAngle => {
                // Set the split angle in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_angle = -ev.new_value.to_radians();
            }
            SliderId::SplitForce => {
                // Set the split force in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_force = ev.new_value;
            }
            SliderId::Daughter1Angle => {
                // Set the daughter 1 split angle in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).daughter_angles.0 = -ev.new_value.to_radians();
            }
            SliderId::Daughter2Angle => {
                // Set the daughter 2 split angle in the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).daughter_angles.1 = -ev.new_value.to_radians();
            }
            SliderId::CellEditorAge => {
                // Set the cell editor age
                editor_state.editor_age.set_age(ev.new_value);
            }
            SliderId::ColourPickerHue => {
                // If there is a target entity set, and it is found in the query
                if let Some(target_entity) = ev.target_entity
                    && let Ok((mut colour_picker, material_handle)) = colour_picker_query.get_mut(target_entity)
                {
                    // Set the hue in the colour picker
                    colour_picker.hue = ev.new_value;

                    // Set the cell colour in the genome mode
                    editor_state.get_selected_genome_mode_mut(&mut genome_bank).colour = colour_picker.to_colour();

                    // If the material can be gotten by its handle
                    if let Some(material) = ui_materials.get_mut(material_handle.id()) {
                        // Set the hue in the colour picker area's material
                        material.hue = colour_picker.hue;
                    }

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            }
        }

        // Clear the simulation cache
        simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
    }
}
