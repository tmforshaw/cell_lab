#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

use bevy::prelude::*;

use crate::{
    cell::{bound_cells, cell_decay, cells_absorb_chemical, cells_do_meiosis, move_cells},
    chemical::{ChemicalTimer, spawn_chemicals},
    input::{keyboard_event_system_cell_editor_mode, keyboard_event_system_play_mode},
    state::{GameMode, GameState, exit_cell_editor_mode, exit_play_mode, init_cell_editor_mode, init_play_mode},
};

pub mod cell;
pub mod chemical;
pub mod dish;
pub mod helpers;
pub mod input;
pub mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_state::<GameMode>()
        .insert_state(GameMode::CellEditor)
        .init_resource::<GameState>()
        .init_resource::<ChemicalTimer>()
        .add_systems(Startup, setup)
        // ---------------------------- Play Mode -----------------------------
        .add_systems(OnEnter(GameMode::Play), init_play_mode)
        .add_systems(
            Update,
            (
                keyboard_event_system_play_mode,
                spawn_chemicals,
                move_cells,
                bound_cells,
                cells_absorb_chemical,
                cells_do_meiosis,
            )
                .run_if(in_state(GameMode::Play)),
        )
        .add_systems(PostUpdate, (cell_decay).run_if(in_state(GameMode::Play)))
        .add_systems(OnExit(GameMode::Play), exit_play_mode)
        // ------------------------- Cell Editor Mode --------------------------
        .add_systems(OnEnter(GameMode::CellEditor), init_cell_editor_mode)
        .add_systems(
            Update,
            (keyboard_event_system_cell_editor_mode,).run_if(in_state(GameMode::CellEditor)),
        )
        .add_systems(OnExit(GameMode::CellEditor), exit_cell_editor_mode)
        // ---------------------------------------------------------------------
        .run();
}

// Spawn cells and chemicals
fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);
}
