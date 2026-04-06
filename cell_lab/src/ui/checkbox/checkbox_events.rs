use bevy::prelude::*;

use crate::{
    cell_editor::{simulation::CellEditorSimulationClearMessage, state::CellEditorState},
    genomes::GenomeBank,
    ui::{CheckboxId, checkbox::checkbox_systems::Checkbox},
};

#[derive(Message)]
pub struct CheckboxEvent {
    pub entity: Entity,
    pub id: CheckboxId,
    pub new_value: bool,
}

pub fn checkbox_event_reader(
    mut events: MessageReader<CheckboxEvent>,
    mut checkboxes: Query<&mut Checkbox>,
    mut editor_state: ResMut<CellEditorState>,
    mut genome_bank: ResMut<GenomeBank>,
    mut simulation_cache_message_writer: MessageWriter<CellEditorSimulationClearMessage>,
) {
    for ev in events.read() {
        match ev.id {
            CheckboxId::InitialMode => {
                // Write the cell split type into the selected genome
                let genome = editor_state.get_selected_genome_mut(&mut genome_bank);

                // If the initial isn't already this mode
                if genome.initial == editor_state.selected_genome_mode {
                    // Make the checkbox selected again
                    if let Ok(mut checkbox) = checkboxes.get_mut(ev.entity) {
                        checkbox.selected = true;
                    }

                    // Don't need to update colours since the checkbox is highlighted so interaction changes will happen
                } else {
                    // Set the initial genome mode to be this mode
                    genome.initial = editor_state.selected_genome_mode;

                    // Clear the simulation cache
                    simulation_cache_message_writer.write(CellEditorSimulationClearMessage);
                }
            }
        }
    }
}
