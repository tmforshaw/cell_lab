use bevy::prelude::*;

use crate::{
    cells::Cell,
    despawning::PendingDespawn,
    game::{game_mode::GameMode, game_parameters::GameParameters},
};

#[derive(Debug, Clone)]
pub struct AdhesionLink {
    pub other: Entity,
    pub dir_to_anchor: Vec2,
    pub rest_length: f32,
    pub strength: f32,
    pub damping: f32,
}

impl AdhesionLink {
    #[must_use]
    pub fn new_from_entity(other: Entity, dir_to_anchor: Vec2) -> Self {
        Self {
            other,
            dir_to_anchor: dir_to_anchor.normalize(),
            rest_length: 1.0,
            strength: 500.,
            damping: 1.0,
        }
    }
}

#[derive(Component, Clone)]
pub struct Adhesion {
    pub links: Vec<AdhesionLink>,
}

#[allow(clippy::needless_pass_by_value)]
pub fn apply_adhesion_system(
    mut adhesions: Query<(Entity, &Cell, &mut Transform, &Adhesion), Without<PendingDespawn>>,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
) {
    const MAX_CORRECTION: f32 = 0.5;
    const SOLVER_ITERATIONS: usize = 5;

    let adhesions_vec = adhesions
        .iter()
        .map(|(entity, _, _, adhesion)| (entity, adhesion.clone()))
        .collect::<Vec<_>>();

    for _ in 0..SOLVER_ITERATIONS {
        for (entity, adhesion) in &adhesions_vec {
            for link in &adhesion.links {
                if let Ok(
                [
                    (entity_1, cell_1, mut transform_1, _adhesion_1),
                    (entity_2, cell_2, mut transform_2, _adhesion_2),
                ],
            ) = adhesions.get_many_mut([*entity, link.other])
                // Enforce an ordering so adhesion resolution isn't repeated
                && entity_1 >= entity_2
                {
                    let world_dir = transform_1.rotation.mul_vec3(link.dir_to_anchor.extend(0.)).xy().normalize();

                    let cell_1_radius = cell_1.get_size(&param, &game_mode).x * 0.5;
                    let cell_2_radius = cell_2.get_size(&param, &game_mode).x * 0.5;

                    let world_anchor_1 = transform_1.translation.xy() + world_dir * cell_1_radius;
                    let world_anchor_2 = transform_2.translation.xy() - world_dir * cell_2_radius;

                    let anchor_correction = (world_anchor_2 - world_anchor_1) * 0.5;

                    // // Remove the component which pushes centres in wrong direction
                    // let centre_delta = transform_2.translation.xy() - transform_1.translation.xy();
                    // let centre_error = centre_delta.dot(world_dir) - (cell_1_radius + cell_2_radius);
                    // let centre_correction = world_dir * centre_error * 0.5;

                    let correction = anchor_correction.clamp_length_max(MAX_CORRECTION);

                    transform_1.translation += correction.extend(0.);
                    transform_2.translation -= correction.extend(0.);
                }
            }
        }
    }
}

// pub fn adhesion_cleanup(mut commands: Commands, adhesion_query: Query<(Entity, &Adhesions), With<PendingDespawn>>) {
//     for (entity, adhesions) in &adhesion_query {
//         // Remove this cell from neighbors’ link lists
//         for &neighbor in &adhesions.links {
//             if let Ok((_, neighbor_adh)) = adhesion_query.get(neighbor) {
//                 commands.entity(neighbor).insert(Adhesions {
//                     links: neighbor_adh.links.iter().filter(|&&x| x != entity).copied().collect(),
//                     params: neighbor_adh.params.clone(),
//                 });
//             }
//         }

//         // TODO
//         commands.entity(entity).despawn();
//     }
// }

#[allow(clippy::needless_pass_by_value)]
pub fn visualise_adhesions(
    mut gizmos: Gizmos,
    adhesions: Query<(&Cell, &Transform, &Adhesion), Without<PendingDespawn>>,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
) {
    for (cell, transform, adhesion) in &adhesions {
        for link in &adhesion.links {
            if let Ok((other, other_transform, _other_adhesion)) = adhesions.get(link.other) {
                let world_dir = transform.rotation.mul_vec3(link.dir_to_anchor.extend(0.)).xy().normalize();

                let cell_1_radius = cell.get_size(&param, &game_mode).x * 0.5;
                let cell_2_radius = other.get_size(&param, &game_mode).x * 0.5;

                let world_anchor_1 = transform.translation.xy() + world_dir * cell_1_radius;
                let world_anchor_2 = other_transform.translation.xy() - world_dir * cell_2_radius;

                gizmos.line(
                    world_anchor_1.extend(4.),
                    world_anchor_2.extend(4.),
                    Color::linear_rgb(0., 0., 1.),
                );
            }
        }
    }
}
