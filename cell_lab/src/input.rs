use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};

use crate::{
    game_mode::GameMode,
    spatial_partitioning::{cell_quadtree::ShowCellQuadTree, chemical_quadtree::ShowChemicalQuadTree},
};

pub fn simulation_mode_keyboard_event_reader(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    for ev in events.read() {
        if ev.state == ButtonState::Pressed {
            #[allow(clippy::single_match)]
            match ev.key_code {
                KeyCode::Digit2 => next_mode.set(GameMode::CellEditor),
                _ => {}
            }
        }
    }
}

pub fn cell_editor_mode_keyboard_event_reader(
    mut events: MessageReader<KeyboardInput>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    for ev in events.read() {
        if ev.state == ButtonState::Pressed {
            #[allow(clippy::single_match)]
            match ev.key_code {
                KeyCode::Digit1 => next_mode.set(GameMode::Simulation),
                _ => {}
            }
        }
    }
}

pub fn mode_independent_keyboard_event_reader(
    mut events: MessageReader<KeyboardInput>,
    mut show_cell_quadtree: ResMut<ShowCellQuadTree>,
    mut show_chemical_quadtree: ResMut<ShowChemicalQuadTree>,
) {
    for ev in events.read() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                KeyCode::BracketRight => show_cell_quadtree.0 = !show_cell_quadtree.0,
                KeyCode::BracketLeft => show_chemical_quadtree.0 = !show_chemical_quadtree.0,
                _ => {}
            }
        }
    }
}
