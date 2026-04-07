use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    cell_editor::state::CellEditorState,
    genomes::GenomeBank,
    serialisation::{does_genome_exist_in_folder, semi_sanitise_filename, write_genome_to_file},
    ui::{TextInputId, UiDialogState, UiWindowId},
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
    mut input_focus: ResMut<InputFocus>,
    editor_state: Res<CellEditorState>,
    genome_bank: Res<GenomeBank>,
) {
    for ev in events.read() {
        match ev.id {
            TextInputId::SaveFilename => {
                // Already done in the text input, but for clarity
                let semi_sanitised_value = semi_sanitise_filename(&ev.new_value);

                // Set the save filename to this value
                dialog_state.save.filename = Some(semi_sanitised_value.clone());

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
                    // Open the overwrite dialog
                    dialog_state.open_dialog(&UiWindowId::OverwriteGenomeDialog);

                    input_focus.clear();
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
