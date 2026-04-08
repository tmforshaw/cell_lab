use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    game::game_parameters::GameParameters,
    genomes::{Genome, GenomeBank, GenomeMode, genome_mode::colour_from_genome_mode_id},
    serialisation::{delete_genome_file, read_genome_file, semi_sanitise_filename, write_genome_to_file},
    ui::{ButtonId, TextInput, UiDialogState, UiWindowId, dialog_events::SaveFilenameEvent, ui_build::UiRebuildState},
};

#[derive(Message)]
pub struct ButtonEvent {
    pub target_entity: Option<Entity>,
    pub id: ButtonId,
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn button_event_reader(
    mut events: MessageReader<ButtonEvent>,
    mut dialog_state: ResMut<UiDialogState>,
    mut input_focus: ResMut<InputFocus>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    param: Res<GameParameters>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,

    mut next_ui_needs_rebuild: ResMut<NextState<UiRebuildState>>,

    text_input_query: Query<&TextInput>,
    mut save_filename_event_writer: MessageWriter<SaveFilenameEvent>,
) {
    for ev in events.read() {
        match ev.id {
            ButtonId::Save => {
                dialog_state.open_dialog(&UiWindowId::SaveGenomeDialog);
            }
            ButtonId::Load => {
                dialog_state.open_dialog(&UiWindowId::LoadGenomeDialog);
            }
            ButtonId::ReplaceModeWithDefault => {
                dialog_state.open_dialog(&UiWindowId::ReplaceModeWithDefaultDialog);
            }
            ButtonId::ConfirmReplaceModeWithDefault => {
                // Make default genome of the correct colour
                let selected_genome = editor_state.get_selected_genome_mode_mut(&mut genome_bank);
                *selected_genome = GenomeMode::new(editor_state.selected_genome_mode);
                selected_genome.colour = colour_from_genome_mode_id(editor_state.selected_genome_mode, &param);

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                // Set the Ui to NeedsRebuild
                next_ui_needs_rebuild.set(UiRebuildState::NeedsRebuild);

                // Close all dialogs
                dialog_state.close_all_dialogs();
            }
            ButtonId::CloseAllDialogs => {
                dialog_state.close_all_dialogs();
            }
            ButtonId::CloseOverwriteGenomeDialog => {
                // Close the overwrite genome dialog
                dialog_state.close_dialog(&UiWindowId::OverwriteGenomeDialog);
            }
            ButtonId::ConfirmOverwriteGenome => {
                // Write genome to file
                if let Some(filename) = &dialog_state.save.filename {
                    write_genome_to_file(filename, editor_state.get_selected_genome(&genome_bank));
                } else {
                    eprintln!("Could not overwrite genome since it was None in dialog_state.save");

                    continue;
                }

                // Exit the dialog
                dialog_state.close_all_dialogs();
            }
            ButtonId::CloseSaveFilenameEmptyDialog => {
                // Close the save filename empty dialog
                dialog_state.close_dialog(&UiWindowId::SaveFilenameIsEmptyDialog);
            }
            ButtonId::SubmitSaveFilename => {
                // If the target entity is specified
                if let Some(target_entity) = ev.target_entity
                    && let Ok(text_input) = text_input_query.get(target_entity)
                {
                    // Semi-sanitise the filename and trigger an event
                    save_filename_event_writer.write(SaveFilenameEvent {
                        filename: semi_sanitise_filename(text_input.value.clone()),
                    });
                }
            }
            ButtonId::ConfirmLoadGenome => {
                // If the load filename is set, and that file can be read from the folder
                if let Some(filename) = dialog_state.load.filename.clone()
                    && let Some(genome) = read_genome_file(&filename)
                {
                    // Set the genome in GenomeBank
                    *editor_state.get_selected_genome_mut(&mut genome_bank) = genome;

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                    // Set the Ui to NeedsRebuild
                    next_ui_needs_rebuild.set(UiRebuildState::NeedsRebuild);
                }

                // Close all the dialogs
                dialog_state.close_all_dialogs();
            }
            ButtonId::DeleteSelectedGenome => {
                // If there is a file selected for deletion
                if dialog_state.load.filename.clone().is_some() {
                    // Open the delete dialog
                    dialog_state.open_dialog(&UiWindowId::DeleteGenomeDialog);
                }
            }
            ButtonId::CloseDeleteDialog => {
                dialog_state.close_dialog(&UiWindowId::DeleteGenomeDialog);
            }
            ButtonId::ConfirmDeleteGenome => {
                // If there is a selected filename to delete
                if let Some(filename) = dialog_state.load.filename.clone() {
                    // Delete the genome
                    delete_genome_file(&filename);

                    // Close all the dialogs
                    dialog_state.close_all_dialogs();
                }
            }
            ButtonId::LoadDefaultGenome => dialog_state.open_dialog(&UiWindowId::LoadDefaultGenomeDialog),
            ButtonId::ConfirmLoadDefaultGenome => {
                // Set the selected genome as the default genome
                *editor_state.get_selected_genome_mut(&mut genome_bank) = Genome::new_from_parameters(&param);

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                // Set the Ui to NeedsRebuild
                next_ui_needs_rebuild.set(UiRebuildState::NeedsRebuild);

                // Close all the dialogs
                dialog_state.close_all_dialogs();
            }
            ButtonId::CloseLoadDefaultGenome => dialog_state.close_dialog(&UiWindowId::LoadDefaultGenomeDialog),
        }

        // Clear input focus
        input_focus.clear();
    }
}
