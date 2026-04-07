use bevy::prelude::*;

use crate::ui::TextInputId;

#[derive(Message)]
pub struct TextInputEvent {
    pub entity: Entity,
    pub id: TextInputId,
    pub new_value: String,
}

pub fn text_input_event_reader(mut events: MessageReader<TextInputEvent>) {
    for ev in events.read() {
        match ev.id {
            TextInputId::SaveFilename => {
                println!("Save filename: '{}'", ev.new_value);
            }
        }
    }
}
