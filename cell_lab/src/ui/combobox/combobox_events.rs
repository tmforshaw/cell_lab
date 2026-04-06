use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::{CellType, GenomeBank, GenomeModeId},
    ui::ComboboxId,
};

#[derive(Message)]
pub struct ComboboxEvent {
    pub id: ComboboxId,
    pub new_value_index: usize,
}

pub fn combobox_event_reader(
    mut events: MessageReader<ComboboxEvent>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) {
    for ev in events.read() {
        match ev.id {
            ComboboxId::SelectedMode | ComboboxId::Daughter1Mode | ComboboxId::Daughter2Mode => {
                let new_mode = Into::<GenomeModeId>::into(ev.new_value_index);

                #[allow(clippy::match_wildcard_for_single_variants)]
                match ev.id {
                    ComboboxId::SelectedMode => {
                        if editor_state.selected_genome_mode != new_mode {
                            // Set the new editor genome mode
                            editor_state.selected_genome_mode = new_mode;
                        }
                    }
                    ComboboxId::Daughter1Mode => {
                        if editor_state.get_selected_genome_mode(&genome_bank).daughter_genome_modes.0 != new_mode {
                            // Set the daughter genome mode
                            editor_state
                                .get_selected_genome_mode_mut(&mut genome_bank)
                                .daughter_genome_modes
                                .0 = new_mode;

                            // Clear the simulation cache
                            simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                        }
                    }
                    ComboboxId::Daughter2Mode => {
                        if editor_state.get_selected_genome_mode(&genome_bank).daughter_genome_modes.1 != new_mode {
                            // Set the daughter genome mode
                            editor_state
                                .get_selected_genome_mode_mut(&mut genome_bank)
                                .daughter_genome_modes
                                .1 = new_mode;

                            // Clear the simulation cache
                            simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                        }
                    }
                    _ => unreachable!(),
                }
            }
            ComboboxId::CellType => {
                let new_cell_type = Into::<CellType>::into(ev.new_value_index);

                if editor_state.get_selected_genome_mode(&genome_bank).cell_type != new_cell_type {
                    // Set the mode's cell type
                    editor_state.get_selected_genome_mode_mut(&mut genome_bank).cell_type = new_cell_type;

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            }
        }
    }
}
