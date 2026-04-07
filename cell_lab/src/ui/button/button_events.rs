use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    game::game_parameters::GameParameters,
    genomes::{GenomeBank, GenomeMode, genome_mode::colour_from_genome_mode_id},
    serialisation::write_genome_to_file,
    ui::{ButtonId, UiDialogState, UiWindowId},
};

#[derive(Message)]
pub struct ButtonEvent {
    pub id: ButtonId,
}

#[allow(clippy::needless_pass_by_value)]
pub fn button_event_reader(
    mut events: MessageReader<ButtonEvent>,
    mut dialog_state: ResMut<UiDialogState>,
    mut input_focus: ResMut<InputFocus>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    param: Res<GameParameters>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) {
    for ev in events.read() {
        match ev.id {
            ButtonId::Save => {
                dialog_state.open_dialog(&UiWindowId::SaveGenomeDialog);

                // Clear input focus
                input_focus.clear();
            }
            ButtonId::Load => {
                dialog_state.open_dialog(&UiWindowId::LoadGenomeDialog);

                // Clear input focus
                input_focus.clear();
            }
            ButtonId::ReplaceModeWithDefault => {
                dialog_state.open_dialog(&UiWindowId::ReplaceModeWithDefaultDialog);

                // Clear input focus
                input_focus.clear();
            }
            ButtonId::ConfirmReplaceModeWithDefault => {
                // Make default genome of the correct colour
                let selected_genome = editor_state.get_selected_genome_mode_mut(&mut genome_bank);
                *selected_genome = GenomeMode::new(editor_state.selected_genome_mode);
                selected_genome.colour = colour_from_genome_mode_id(editor_state.selected_genome_mode, &param);

                // Clear the simulation cache
                simulation_cache_message_writer.write(CellEditorSimulationClearMessage);

                // Close all dialogs
                dialog_state.close_all_dialogs();

                // Clear input focus
                input_focus.clear();
            }
            ButtonId::CloseAllDialogs => {
                dialog_state.close_all_dialogs();

                // Clear input focus
                input_focus.clear();
            }
            ButtonId::CloseOverwriteGenomeDialog => {
                // Close the overwrite genome dialog
                dialog_state.close_dialog(&UiWindowId::OverwriteGenomeDialog);

                // Clear input focus
                input_focus.clear();
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

                // Clear input focus
                input_focus.clear();
            }
        }
    }
}
