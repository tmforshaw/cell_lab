use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{Cell, Velocity},
    spatial_partitioning::quadtree::QuadTreeNode,
};

const IMPULSE_STRENGTH_SCALE_FACTOR: f32 = 10.;

#[must_use]
pub fn aabb_contains_point(aabb: &Aabb2d, p: Vec2) -> bool {
    p.x >= aabb.min.x && p.x <= aabb.max.x && p.y >= aabb.min.y && p.y <= aabb.max.y
}

pub fn collision_system(mut cells: Query<(Entity, &Cell, &mut Velocity, &mut Transform)>) {
    // Build read-only values for use in quadtree
    let mut entities_and_transforms = Vec::new();
    for (e, _cell, _vel, &t) in &cells {
        entities_and_transforms.push((e, t));
    }

    let root = QuadTreeNode::build_tree(entities_and_transforms.clone());

    for (entity, transform) in entities_and_transforms {
        let position = transform.translation.xy();
        let bounds = Aabb2d::new(position, transform.scale.xy());

        let mut candidates = Vec::new();

        root.query(&bounds, &mut candidates);

        for other in candidates {
            if other == entity {
                continue;
            }

            // Check for collision against this other entity
            if let Ok(
                [
                    (_entity, cell, mut velocity, mut transform),
                    (_other_entity, other_cell, mut other_velocity, mut other_transform),
                ],
            ) = cells.get_many_mut([entity, other])
            {
                // Cells are colliding
                let delta = transform.translation - other_transform.translation;
                let dist = delta.length();
                let combined_radius = f32::midpoint(transform.scale.x, other_transform.scale.x);

                if dist < combined_radius {
                    // Utilise mass to get a conservation of momentum-style correction
                    let mass = cell.get_mass();
                    let other_mass = other_cell.get_mass();
                    let combined_mass = mass + other_mass;

                    let mass_ratio = other_mass / combined_mass;
                    let other_mass_ratio = mass / combined_mass;

                    // The direction of the interaction, and the overlap amount
                    let dir = delta.normalize_or_zero();
                    let overlap = combined_radius - dist;

                    // Get the full correction vector
                    let correction = dir * overlap;

                    // Positional correction of the cells
                    transform.translation += correction * mass_ratio;
                    other_transform.translation -= correction * other_mass_ratio;

                    // Define the impulse strength based on the overlap amount
                    let impulse_strength = overlap * IMPULSE_STRENGTH_SCALE_FACTOR;

                    // Add impulse velocities to the cells // TODO Probably clamp these velocities
                    velocity.0 += (dir * impulse_strength * mass_ratio).xy();
                    other_velocity.0 -= (dir * impulse_strength * other_mass_ratio).xy();
                }
            }
        }
    }
}
