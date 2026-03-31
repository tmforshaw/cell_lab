use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};

use crate::{spatial_partitioning::cell_quadtree::ShowCellQuadTree, state::GameMode};

pub fn play_mode_keyboard_event_reader(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mut show_cell_quadtree: ResMut<ShowCellQuadTree>,
) {
    for ev in events.read() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                KeyCode::Digit2 => next_mode.set(GameMode::CellEditor),
                KeyCode::BracketRight => show_cell_quadtree.0 = !show_cell_quadtree.0,
                _ => {}
            }
        }
    }
}

pub fn cell_editor_mode_keyboard_event_reader(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mut show_cell_quadtree: ResMut<ShowCellQuadTree>,
) {
    for ev in events.read() {
        #[allow(clippy::single_match)]
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                KeyCode::Digit1 => next_mode.set(GameMode::Play),
                KeyCode::BracketRight => show_cell_quadtree.0 = !show_cell_quadtree.0,
                _ => {}
            }
        }
    }
}
