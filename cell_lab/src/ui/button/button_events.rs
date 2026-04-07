use bevy::prelude::*;

use crate::ui::{ButtonId, UiDialogState, UiWindowId};

#[derive(Message)]
pub struct ButtonEvent {
    pub id: ButtonId,
}

pub fn button_event_reader(mut events: MessageReader<ButtonEvent>, mut dialog_state: ResMut<UiDialogState>) {
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
            ButtonId::CloseAllDialogs => {
                dialog_state.close_all_dialogs();
            }
        }
    }
}
