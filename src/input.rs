use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::state::GameMode;

pub fn keyboard_event_system_play_mode(mut events: MessageReader<KeyboardInput>, mut next_mode: ResMut<NextState<GameMode>>) {
    for ev in events.read() {
        // TODO
        #[allow(clippy::single_match)]
        match ev.key_code {
            KeyCode::Digit2 => next_mode.set(GameMode::CellEditor),
            _ => {}
        }
    }
}

pub fn keyboard_event_system_cell_editor_mode(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    for ev in events.read() {
        // TODO
        #[allow(clippy::single_match)]
        match ev.key_code {
            KeyCode::Digit1 => next_mode.set(GameMode::Play),
            _ => {}
        }
    }
}
