use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::{CellSplitType, GenomeBank},
    serialisation::get_genomes_in_folder_underscore_to_spaces,
    ui::{RadioId, TextInput, UiDialogState},
};

#[derive(Message)]
pub struct RadioEvent {
    pub target_entity: Option<Entity>,
    pub id: RadioId,
    pub new_value_index: Option<usize>,
}

pub fn radio_event_reader(
    mut events: MessageReader<RadioEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut dialog_state: ResMut<UiDialogState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,

    mut text_input_query: Query<&mut TextInput>,
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
                // Ensure that there actually is a selected index, and the target entity is set
                if let Some(selected_index) = ev.new_value_index
                    && let Some(target_entity) = ev.target_entity
                {
                    // Get the genomes which are saved in the folder, and select the correct genome
                    if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces()
                        && let Some(selected_genome) = genomes.get(selected_index)
                    {
                        // Get the text input from its entity ID
                        if let Ok(mut text_input) = text_input_query.get_mut(target_entity) {
                            // Copy this name into the text input
                            text_input.value = (**selected_genome).clone();
                        }
                    }
                }
            }
            RadioId::LoadFileNames => {
                // Ensure that there actually is a selected index
                if let Some(selected_index) = ev.new_value_index {
                    // Get the genomes which are saved in the folder, and select the correct genome
                    if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces()
                        && let Some(selected_genome) = genomes.get(selected_index)
                    {
                        // Set the filename to load in the dialog state
                        dialog_state.load.filename = Some(selected_genome.clone());
                    }
                }
            }
        }
    }
}
