use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::state::GameMode;

pub fn keyboard_event_system(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mode: Res<State<GameMode>>,
) {
    for ev in events.read() {
        match ev.key_code {
            KeyCode::Digit1 if !mode.eq(&GameMode::Play) => next_mode.set(GameMode::Play),
            KeyCode::Digit2 if !mode.eq(&GameMode::CellEditor) => next_mode.set(GameMode::CellEditor),
            _ => {}
        }
    }
}
