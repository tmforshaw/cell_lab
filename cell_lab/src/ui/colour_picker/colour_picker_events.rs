use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::GenomeBank,
    ui::ColourPickerId,
};

#[derive(Message)]
pub struct ColourPickerEvent {
    pub id: ColourPickerId,
    pub new_value: Color,
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn colour_picker_event_reader(
    mut events: MessageReader<ColourPickerEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,

    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) {
    for ev in events.read() {
        match ev.id {
            ColourPickerId::SelectedCellColour => {
                // Set the cell colour in the genome mode
                editor_state.get_selected_genome_mode_mut(&mut genome_bank).colour = ev.new_value;

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }
        }
    }
}
