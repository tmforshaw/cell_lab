use bevy::prelude::*;

use crate::{cell_editor::state::CellEditorState, genomes::GenomeModeId, ui::ComboboxId};

#[derive(Message)]
pub struct ComboboxEvent {
    pub id: ComboboxId,
    pub new_value_index: usize,
}

pub fn combobox_event_reader(mut events: MessageReader<ComboboxEvent>, mut editor_state: ResMut<CellEditorState>) {
    for ev in events.read() {
        match ev.id {
            ComboboxId::Mode => {
                let new_mode = Into::<GenomeModeId>::into(ev.new_value_index);

                if editor_state.selected_genome_mode != new_mode {
                    // Set the new editor genome mode
                    editor_state.selected_genome_mode = new_mode;
                }
            }
        }
    }
}
