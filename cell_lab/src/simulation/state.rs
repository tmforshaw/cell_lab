use bevy::prelude::*;

use crate::{
    cells::{CELL_MAX_VELOCITY, CELL_STARTING_ENERGY, Cell, CellMaterial, STARTING_CELL_NUM},
    genomes::{GenomeBankId, GenomeCollection, GenomeId},
    helpers::random_vec2,
    simulation::{
        chemical::Chemical,
        dish::{Dish, DishMarker},
    },
};

const SIMULATION_SIZE: Vec2 = Vec2::splat(1600.);
const SIMULATION_CELL_SIZE_PER_MASS: f32 = 10.;

#[derive(Resource)]
pub struct SimulationState {
    pub dish: Dish,
    pub cell_size_per_mass: f32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            dish: Dish::new(SIMULATION_SIZE),
            cell_size_per_mass: SIMULATION_CELL_SIZE_PER_MASS,
        }
    }
}

impl SimulationState {
    #[must_use]
    pub fn new(dish: Dish) -> Self {
        Self { dish, ..default() }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn init_simulation_mode(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    state: Res<SimulationState>,
) {
    // Insert the state resource
    commands.insert_resource(SimulationState::default());

    // Show dish
    commands.spawn(state.dish.into_bundle());

    // Spawn cells
    for _ in 0..STARTING_CELL_NUM {
        commands.spawn(Cell::new_bundle(
            CELL_STARTING_ENERGY,
            GenomeId::default(),
            GenomeBankId::default(),
            state.cell_size_per_mass,
            random_vec2(Vec2::splat(CELL_MAX_VELOCITY)),
            random_vec2(state.dish.size / 2.),
            &genome_collection,
            &mut meshes,
            &mut materials,
        ));
    }
}

pub fn exit_simulation_mode(
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
