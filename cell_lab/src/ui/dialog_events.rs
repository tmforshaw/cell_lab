use bevy::prelude::*;

use crate::{
    cell_editor::state::CellEditorState,
    genomes::GenomeBank,
    helpers::SemiSanitisedString,
    serialisation::{does_genome_exist_in_folder, write_genome_to_file},
    ui::{UiDialogState, UiWindowId},
};

#[derive(Message)]
pub struct SaveFilenameEvent {
    pub filename: SemiSanitisedString,
}

#[allow(clippy::needless_pass_by_value)]
pub fn save_filename_event_reader(
    mut events: MessageReader<SaveFilenameEvent>,
    mut dialog_state: ResMut<UiDialogState>,
    editor_state: Res<CellEditorState>,
    genome_bank: Res<GenomeBank>,
) {
    for ev in events.read() {
        // Set the save filename to this value
        dialog_state.save.filename = Some(ev.filename.clone());

        // Check if the file already exists
        if does_genome_exist_in_folder(&ev.filename) {
            // Open the overwrite dialog
            dialog_state.open_dialog(&UiWindowId::OverwriteGenomeDialog);
        } else if ev.filename.trim().is_empty() {
            // Open save filename is empty dialog
            dialog_state.open_dialog(&UiWindowId::SaveFilenameIsEmptyDialog);
        } else {
            // Write genome to file
            write_genome_to_file(&ev.filename, editor_state.get_selected_genome(&genome_bank));

            // Exit the dialog
            dialog_state.close_all_dialogs();
        }
    }
}
