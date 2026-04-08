use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    serialisation::semi_sanitise_filename,
    ui::{TextInputId, dialog_events::SaveFilenameEvent},
};

#[derive(Message)]
pub struct TextInputEvent {
    pub id: TextInputId,
    pub new_value: String,
}

#[allow(clippy::needless_pass_by_value)]
pub fn text_input_event_reader(
    mut events: MessageReader<TextInputEvent>,
    mut input_focus: ResMut<InputFocus>,
    mut save_filename_event_writer: MessageWriter<SaveFilenameEvent>,
) {
    for ev in events.read() {
        match ev.id {
            TextInputId::SaveFilename => {
                // Semi-sanitise the filename and trigger an event
                save_filename_event_writer.write(SaveFilenameEvent {
                    filename: semi_sanitise_filename(&ev.new_value),
                });
            }
        }

        // Clear the input focus
        input_focus.clear();
    }
}
