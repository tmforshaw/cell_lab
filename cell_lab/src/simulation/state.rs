use bevy::prelude::*;

use crate::{
    cells::{Cell, CellMaterial},
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::{GenomeBank, GenomeId, GenomeModeId},
    helpers::random_vec2,
    simulation::{chemical::Chemical, dish::DishMarker},
    ui::UiRebuildState,
};

#[allow(clippy::needless_pass_by_value)]
pub fn init_simulation_mode(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    genome_bank: Res<GenomeBank>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
) {
    // Show dish
    commands.spawn(param.simulation_mode.dish_parameters.get_dish_bundle());

    // Set the Ui to NeedsRebuild
    commands.set_state(UiRebuildState::NeedsRebuild);

    // Spawn cells
    for _ in 0..param.simulation_mode.starting_cell_num {
        commands.spawn(Cell::new_bundle(
            param.cell_parameters.starting_energy,
            GenomeModeId::default(),
            GenomeId::default(),
            &param,
            &game_mode,
            random_vec2(Vec2::splat(param.cell_parameters.max_velocity)),
            random_vec2(param.simulation_mode.dish_parameters.size / 2.),
            &genome_bank,
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
