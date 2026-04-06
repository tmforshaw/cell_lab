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

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }
        }
    }
}
