use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cell_editor::state::{CELL_EDITOR_CELL_SIZE_PER_MASS, CellEditorState},
    cells::{CELL_STARTING_ENERGY, Cell, Velocity},
    collision::systems::resolve_cell_collision,
    despawning::PendingDespawn,
    genomes::GenomeBank,
    spatial_partitioning::{
        cell_quadtree::{CELL_QUADTREE_MAX_CAPACITY_PER_NODE, CELL_QUADTREE_MAX_DEPTH},
        quadtree::QuadTree,
    },
};

pub const PHYSICS_STEP_DT: f32 = 0.02;

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
pub fn create_root_logical_cell(state: &CellEditorState, genome_bank: &GenomeBank) -> LogicalCell {
    let cell = Cell {
        energy: CELL_STARTING_ENERGY,
        age: 0.0,
        genome_mode_id: state.get_selected_genome(genome_bank).initial,
        genome_id: state.selected_genome,
        size_per_mass: CELL_EDITOR_CELL_SIZE_PER_MASS,
    };

    LogicalCell {
        cell: cell.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(cell.get_size().extend(1.0)),
        velocity: Velocity(Vec2::ZERO),
        time_of_birth: 0.0,
    }
}

pub fn resolve_logical_cell_collision(cells: &mut [LogicalCell], editor_size: Vec2) {
    // Build quadtree from current cell positions
    let identifiers_and_transforms: Vec<(usize, Transform)> = cells.iter().enumerate().map(|(i, lc)| (i, lc.transform)).collect();

    let qt = QuadTree::build_new(
        Vec2::ZERO,
        editor_size,
        CELL_QUADTREE_MAX_DEPTH,
        CELL_QUADTREE_MAX_CAPACITY_PER_NODE,
        &identifiers_and_transforms.iter().map(|(i, t)| (*i, *t)).collect(),
    );

    for i in 0..cells.len() {
        let mut neighbours = Vec::new();
        let lc = &cells[i];
        let aabb = Aabb2d::new(lc.transform.translation.xy(), lc.cell.get_size());
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
            );
        }
    }
}
