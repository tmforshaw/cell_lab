use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{CELL_MAX_VELOCITY, Cell, Velocity},
    despawning::PendingDespawn,
    spatial_partitioning::cell_quadtree::CellQuadTree,
};

const IMPULSE_STRENGTH_SCALE_FACTOR: f32 = 10.;

#[must_use]
pub fn aabb_contains_point(aabb: &Aabb2d, p: Vec2) -> bool {
    p.x >= aabb.min.x && p.x <= aabb.max.x && p.y >= aabb.min.y && p.y <= aabb.max.y
}

pub fn resolve_cell_collision(
    cell1: &Cell,
    cell1_transform: &mut Transform,
    cell1_velocity: &mut Velocity,
    cell2_cell: &Cell,
    cell2_transform: &mut Transform,
    cell2_velocity: &mut Velocity,
) {
    // Get the displacement and combined radius for the cells
    let displacement = (cell1_transform.translation - cell2_transform.translation).xy();
    let distance = displacement.length();
    let combined_radius = f32::midpoint(cell1_transform.scale.x, cell2_transform.scale.x); // Use X since X and Y scale should be identical

    // Cells are overlapping
    if distance < combined_radius {
        // Utilise mass to get a conservation of momentum-style correction
        let cell1_mass = cell1.get_mass();
        let cell2_mass = cell2_cell.get_mass();
        let combined_mass = cell1_mass + cell2_mass;

        // Get the mass ratios to perform momentum conservation (Use other cell's mass to give correct result)
        let cell1_mass_ratio = cell2_mass / combined_mass;
        let cell2_mass_ratio = cell1_mass / combined_mass;

        // The direction of the interaction, and the overlap amount
        let dir = displacement.normalize_or_zero();
        let overlap = combined_radius - distance;

        // Get the full correction vector
        let correction = (dir * overlap).extend(0.);

        // Positional correction of the cells
        cell1_transform.translation += correction * cell1_mass_ratio;
        cell2_transform.translation -= correction * cell2_mass_ratio;

        // Define the impulse strength based on the overlap amount
        let impulse_strength = overlap * IMPULSE_STRENGTH_SCALE_FACTOR;

        // Add impulse velocities to the cells
        cell1_velocity.0 += (dir * impulse_strength * cell1_mass_ratio).clamp_length_max(CELL_MAX_VELOCITY);
        cell2_velocity.0 -= (dir * impulse_strength * cell2_mass_ratio).clamp_length_max(CELL_MAX_VELOCITY);
    }
}

pub fn collision_system(
    mut cell_quadtree: ResMut<CellQuadTree>,
    mut cells: Query<(Entity, &Cell, &mut Transform, &mut Velocity), Without<PendingDespawn>>,
) {
    // Create a read-only Vec so that the collision resolution can borrow 'cells' mutably
    let mut entities_and_transforms = Vec::new();
    for (entity, _cell, &transform, _velocity) in &cells {
        entities_and_transforms.push((entity, transform));
    }

    // Build the cell quadtree, and get the root node
    *cell_quadtree = CellQuadTree::default();
    cell_quadtree.0.build(&entities_and_transforms);
    let root = cell_quadtree.0.get_root();

    for (entity, transform) in entities_and_transforms {
        let mut candidates = Vec::new();

        // Create the bounding box for a collision with this cell, then query candidates for collision in the quadtree
        let bounds = Aabb2d::new(transform.translation.xy(), transform.scale.xy());
        root.query(&bounds, &mut candidates);

        // Remove candidates which are identical to this entity, then resolve collisions
        for other in candidates {
            // Other is the same as this entity
            if other == entity {
                continue;
            }

            // Check for collision against the other entity
            if let Ok(
                [
                    (_entity, cell1, mut cell1_transform, mut cell1_velocity),
                    (_cell2_entity, cell2, mut cell2_transform, mut cell2_velocity),
                ],
            ) = cells.get_many_mut([entity, other])
            {
                // Cells are colliding
                resolve_cell_collision(
                    cell1,
                    &mut cell1_transform,
                    &mut cell1_velocity,
                    cell2,
                    &mut cell2_transform,
                    &mut cell2_velocity,
                );
            }
        }
    }
}
