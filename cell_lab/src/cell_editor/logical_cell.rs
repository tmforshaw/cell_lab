use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cell_editor::state::CellEditorState,
    cells::{Cell, Velocity},
    collision::systems::resolve_cell_collision,
    despawning::PendingDespawn,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::GenomeBank,
    spatial_partitioning::quadtree::QuadTree,
};

#[derive(Debug, Clone)]
pub struct LogicalCell {
    pub cell: Cell,
    pub transform: Transform,
    pub velocity: Velocity,
    pub time_of_birth: f32,
}
pub fn clear_cells(mut commands: Commands, cells: Query<Entity, (With<Cell>, Without<PendingDespawn>)>) {
    for cell in cells {
        commands.entity(cell).insert(PendingDespawn);
    }
}

#[must_use]
pub fn create_root_logical_cell(
    state: &CellEditorState,
    param: &GameParameters,
    game_mode: &GameMode,
    genome_bank: &GenomeBank,
) -> LogicalCell {
    let cell = Cell {
        energy: param.cell_parameters.starting_energy,
        age: 0.0,
        genome_mode_id: state.get_selected_genome(genome_bank).initial,
        genome_id: state.selected_genome,
    };

    LogicalCell {
        cell: cell.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(cell.get_size(param, game_mode).extend(1.0)),
        velocity: Velocity(Vec2::ZERO),
        time_of_birth: 0.0,
    }
}

pub fn resolve_logical_cell_collision(cells: &mut [LogicalCell], param: &GameParameters, game_mode: &GameMode) {
    // Build quadtree from current cell positions
    let identifiers_and_transforms: Vec<(usize, Transform)> = cells.iter().enumerate().map(|(i, lc)| (i, lc.transform)).collect();

    let qt = QuadTree::build_new(
        Vec2::ZERO,
        param.cell_editor_mode.dish_parameters.size,
        param.cell_editor_mode.cell_quadtree.max_depth,
        param.cell_editor_mode.cell_quadtree.max_capacity_per_node,
        &identifiers_and_transforms.iter().map(|(i, t)| (*i, *t)).collect(),
    );

    for i in 0..cells.len() {
        let mut neighbours = Vec::new();
        let lc = &cells[i];
        let aabb = Aabb2d::new(lc.transform.translation.xy(), lc.cell.get_size(param, game_mode));
        qt.root.query(&aabb, &mut neighbours);

        for &neighbour_id in &neighbours {
            if neighbour_id <= i {
                continue; // Avoid double-processing
            }

            // Split queue into two non-overlapping slices
            let (left, right) = cells.split_at_mut(neighbour_id);

            // Break LogicalCell into fields
            let lc1 = &mut left[i];
            let cell_1 = &lc1.cell;
            let cell1_transform = &mut lc1.transform;
            let cell1_velocity = &mut lc1.velocity;

            // Break LogicalCell into fields
            let neighbour = &mut right[0];
            let cell_2 = &neighbour.cell;
            let cell2_transform = &mut neighbour.transform;
            let cell2_velocity = &mut neighbour.velocity;

            resolve_cell_collision(
                cell_1,
                cell1_transform,
                cell1_velocity,
                cell_2,
                cell2_transform,
                cell2_velocity,
                param,
            );
        }
    }
}
