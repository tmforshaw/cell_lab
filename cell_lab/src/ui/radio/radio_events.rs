use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::{CellSplitType, GenomeBank},
    serialisation::get_genomes_in_folder_underscore_to_spaces,
    ui::RadioId,
};

#[derive(Message)]
pub struct RadioEvent {
    pub id: RadioId,
    pub new_value_index: Option<usize>,
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
                if let Some(selected_index) = ev.new_value_index {
                    editor_state.get_selected_genome_mode_mut(&mut genome_bank).split_type =
                        Into::<CellSplitType>::into(selected_index);
                }

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
            }
            RadioId::SaveFileNames => {
                // Ensure that there actually is a selected index
                if let Some(selected_index) = ev.new_value_index {
                    // Get the genomes which are saved in the folder
                    if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces() {
                        // Select the genome
                        if let Some(selected_genome) = genomes.get(selected_index) {
                            // TODO Copy this name into the text input
                            let _genome_string = (**selected_genome).clone();
                        }
                    }
                }
            }
        }
    }
}
