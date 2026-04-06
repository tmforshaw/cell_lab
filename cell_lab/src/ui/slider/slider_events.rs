use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::GenomeBank,
    ui::SliderId,
};

#[derive(Message)]
pub struct SliderEvent {
    pub id: SliderId,
    pub new_value: f32,
}

pub fn slider_event_reader(
    mut events: MessageReader<SliderEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
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
        }

        // Clear the simulation cache
        simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
    }
}
