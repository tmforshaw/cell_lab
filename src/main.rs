use bevy::prelude::*;

use crate::{
    cell::{
        CELL_ENERGY, CELL_MAX_VELOCITY, Cell, STARTING_CELL_NUM, bound_cells, cell_decay, cells_absorb_chemical,
        cells_do_meiosis, move_cells,
    },
    chemical::{ChemicalTimer, spawn_chemicals},
    helpers::random_vec2,
    state::{GameState, play_mode_criteria},
};

pub mod cell;
pub mod chemical;
pub mod dish;
pub mod helpers;
pub mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .init_resource::<ChemicalTimer>()
        .add_systems(Startup, setup)
        // ---------------------------- Play Mode -----------------------------
        .add_systems(
            Update,
            (
                spawn_chemicals,
                move_cells,
                bound_cells,
                cells_absorb_chemical,
                cells_do_meiosis,
            )
                .run_if(play_mode_criteria),
        )
        .add_systems(PostUpdate, (cell_decay).run_if(play_mode_criteria))
        // ---------------------------------------------------------------------
        .run();
}

// Spawn cells and chemicals
fn setup(mut commands: Commands, state: Res<GameState>) {
    // 2D camera
    commands.spawn(Camera2d);

    // Show dish
    commands.spawn(state.dish.into_sprite());

    // Spawn cells
    for _ in 0..STARTING_CELL_NUM {
        commands.spawn(Cell::new_bundle(
            CELL_ENERGY,
            random_vec2(Vec2::splat(CELL_MAX_VELOCITY)),
            random_vec2(state.dish.size / 2.),
            Color::linear_rgb(0., 1., 0.),
        ));
    }
}
