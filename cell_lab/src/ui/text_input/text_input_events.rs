use bevy::prelude::*;

use crate::{
    cell_editor::state::CellEditorState,
    genomes::GenomeBank,
    serialisation::{does_genome_exist_in_folder, semi_sanitise_filename, write_genome_to_file},
    ui::{TextInputId, UiDialogState},
};

#[derive(Message)]
pub struct TextInputEvent {
    pub id: TextInputId,
    pub new_value: String,
}

#[allow(clippy::needless_pass_by_value)]
pub fn text_input_event_reader(
    mut events: MessageReader<TextInputEvent>,
    mut dialog_state: ResMut<UiDialogState>,
    editor_state: Res<CellEditorState>,
    genome_bank: Res<GenomeBank>,
) {
    for ev in events.read() {
        match ev.id {
            TextInputId::SaveFilename => {
                println!("Save filename: '{}'", ev.new_value);

                // Already done in the text input, but for clarity
                let semi_sanitised_value = semi_sanitise_filename(&ev.new_value);

                // TODO For selecting genome in the list of genomes that already exist
                // // Check if there are genomes with this name already
                // if let Some(genomes) = get_genomes_in_folder_underscore_to_spaces()
                //     && let Some((i, _)) = genomes
                //         .iter()
                //         .enumerate()
                //         .find(|&(_, genome)| genome == &semi_sanitised_value)
                // {
                //     // Mark this genome as the selected one
                //     dialog_state.save.selected_genome = Some(i);
                // } else {
                //     // Clear the selected genome
                //     dialog_state.save.selected_genome = None;
                // }

                // Check if the file already exists
                if does_genome_exist_in_folder(&semi_sanitised_value) {
                    // TODO Open the overwrite dialog
                    todo!()
                } else if semi_sanitised_value.trim().is_empty() {
                    // TODO Open filename was empty dialog
                    todo!()
                } else {
                    // Write genome to file
                    write_genome_to_file(&semi_sanitised_value, editor_state.get_selected_genome(&genome_bank));

                    // Exit the dialog
                    dialog_state.close_all_dialogs();
                }
            }
        }
    }
}
