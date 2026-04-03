use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{Cell, Velocity},
    despawning::PendingDespawn,
    game::game_parameters::GameParameters,
    spatial_partitioning::cell_quadtree::CellQuadTree,
};

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
    param: &GameParameters,
) {
    // Get the displacement and combined radius for the cells
    let displacement = (cell1_transform.translation - cell2_transform.translation).xy();
    let distance = displacement.length();
    let combined_radius = f32::midpoint(cell1_transform.scale.x, cell2_transform.scale.x); // Use X since X and Y scale should be identical

    // Cells are overlapping
    if distance < combined_radius {
        // Utilise mass to get a conservation of momentum-style correction
        let cell1_mass = cell1.get_mass(param);
        let cell2_mass = cell2_cell.get_mass(param);
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
        let impulse_strength = overlap * param.collision_impulse_scale;

        // Add impulse velocities to the cells
        cell1_velocity.0 += dir * impulse_strength * cell1_mass_ratio;
        cell2_velocity.0 -= dir * impulse_strength * cell2_mass_ratio;

        // Clamp the total velocity
        cell1_velocity.0 = cell1_velocity.0.clamp_length_max(param.cell_parameters.max_velocity);
        cell2_velocity.0 = cell2_velocity.0.clamp_length_max(param.cell_parameters.max_velocity);

        // Check to see if cells are overlapping still, then apply some momentum in the normal direction
        let new_displacement = cell1_transform.translation - cell2_transform.translation;
        let new_overlap = combined_radius - new_displacement.length();
        if new_overlap > 0.0 {
            // Get the perpendicular direction w.r.t the vector between both cells
            let normal_dir = new_displacement.normalize().cross(Vec3::Z).xy();

            // Correct the position in the normal direction, with the new overlap as the magnitude
            let correction = (normal_dir * new_overlap).extend(0.);

            // Additional positional correction of the cells
            cell1_transform.translation += correction * cell1_mass_ratio;
            cell2_transform.translation -= correction * cell2_mass_ratio;

            // Calculate additional velocity in the normal direction
            let new_impulse_strength = new_overlap * param.collision_impulse_scale;

            // Add addditional impulse velocities to the cells
            cell1_velocity.0 += normal_dir * new_impulse_strength * cell1_mass_ratio;
            cell2_velocity.0 -= normal_dir * new_impulse_strength * cell2_mass_ratio;

            // Clamp the total velocity
            cell1_velocity.0 = cell1_velocity.0.clamp_length_max(param.cell_parameters.max_velocity);
            cell2_velocity.0 = cell2_velocity.0.clamp_length_max(param.cell_parameters.max_velocity);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn collision_system(
    cell_quadtree: Res<CellQuadTree>,
    param: Res<GameParameters>,
    mut cells: Query<(Entity, &Cell, &mut Transform, &mut Velocity), Without<PendingDespawn>>,
) {
    // Create a read-only Vec so that the collision resolution can borrow 'cells' mutably
    let mut entities_and_transforms = Vec::new();
    for (entity, _cell, &transform, _velocity) in &cells {
        entities_and_transforms.push((entity, transform));
    }

    // Assume that quadtree is already built, so just get root
    let root = cell_quadtree.0.get_root();

    for (entity, transform) in entities_and_transforms {
        let mut candidates = Vec::new();

        // Create the bounding box for a collision with this cell, then query candidates for collision in the quadtree (Don't half the size, to include neighbouring quadrants)
        let bounds = Aabb2d::new(transform.translation.xy(), transform.scale.xy());
        root.query(&bounds, &mut candidates);

        // Iterate through coarse collision detection candidates
        for other in candidates {
            // Don't consider if other == this cell
            if other == entity {
                continue;
            }

            // Access data for this cell and the other cell, then perform fine collision detection and resolution
            if let Ok(
                [
                    (_entity, cell1, mut cell1_transform, mut cell1_velocity),
                    (_cell2_entity, cell2, mut cell2_transform, mut cell2_velocity),
                ],
            ) = cells.get_many_mut([entity, other])
            {
                // Check if cells are colliding, and resolve that collision
                resolve_cell_collision(
                    cell1,
                    &mut cell1_transform,
                    &mut cell1_velocity,
                    cell2,
                    &mut cell2_transform,
                    &mut cell2_velocity,
                    &param,
                );
            }
        }
    }
}
