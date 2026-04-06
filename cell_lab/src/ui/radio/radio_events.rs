use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::{CellSplitType, GenomeBank},
    ui::RadioId,
};

#[derive(Message)]
pub struct RadioEvent {
    pub id: RadioId,
    pub new_value_index: usize,
}

pub fn radio_event_reader(
    mut events: MessageReader<RadioEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) {
    for ev in events.read() {
        match ev.id {
            RadioId::SplitType => {
                // Write the cell split type into the selected genome
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_type =
                    Into::<CellSplitType>::into(ev.new_value_index);

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }
        }
    }
}
