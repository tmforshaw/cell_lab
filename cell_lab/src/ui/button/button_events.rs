use bevy::prelude::*;

use crate::{cell_editor::state::CellEditorState, ui::ButtonId};

#[derive(Message)]
pub struct ButtonEvent {
    pub id: ButtonId,
}

pub fn button_event_reader(mut events: MessageReader<ButtonEvent>, mut editor_state: ResMut<CellEditorState>) {
    for ev in events.read() {
        match ev.id {
            ButtonId::Save => {
                editor_state.dialogs.open_save_dialog();
            }
            ButtonId::Load => {
                editor_state.dialogs.open_load_dialog();
            }
            ButtonId::ReplaceModeWithDefault => {
                editor_state.dialogs.open_default_genome_mode_dialog();
            }
        }
    }
}
