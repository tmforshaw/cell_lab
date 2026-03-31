use bevy::prelude::*;

use crate::{
    cells::{CELL_ENERGY, CELL_MAX_VELOCITY, Cell, CellMaterial, STARTING_CELL_NUM},
    genomes::{GenomeBankId, GenomeCollection, GenomeId},
    helpers::random_vec2,
    simulation::{
        chemical::Chemical,
        dish::{Dish, DishMarker},
    },
};

#[derive(Resource, Default)]
pub struct PlayModeState {
    pub dish: Dish,
}

impl PlayModeState {
    #[must_use]
    pub const fn new(dish: Dish) -> Self {
        Self { dish }
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum GameMode {
    #[default]
    Play,
    CellEditor,
}

// ---------------------------- Play Mode -----------------------------
#[allow(clippy::needless_pass_by_value)]
pub fn init_play_mode(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    state: Res<PlayModeState>,
) {
    // Show dish
    commands.spawn(state.dish.into_bundle());

    // Spawn cells
    for _ in 0..STARTING_CELL_NUM {
        commands.spawn(Cell::new_bundle(
            CELL_ENERGY,
            GenomeId::default(),
            GenomeBankId::default(),
            random_vec2(Vec2::splat(CELL_MAX_VELOCITY)),
            random_vec2(state.dish.size / 2.),
            &genome_collection,
            &mut meshes,
            &mut materials,
        ));
    }
}

pub fn exit_play_mode(
    mut commands: Commands,
    dishes: Query<Entity, With<DishMarker>>,
    cells: Query<Entity, With<Cell>>,
    chemicals: Query<Entity, With<Chemical>>,
) {
    // Remove the dish
    for entity in dishes {
        commands.entity(entity).despawn();
    }

    // Remove the cells
    for entity in cells {
        commands.entity(entity).despawn();
    }

    // Remove the chemicals
    for entity in chemicals {
        commands.entity(entity).despawn();
    }
}
