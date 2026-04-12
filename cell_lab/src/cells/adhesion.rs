use bevy::prelude::*;

use crate::{
    cells::{Cell, Velocity, cell::CellBundle},
    despawning::PendingDespawn,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::GenomeMode,
};

#[derive(Debug, Clone)]
pub struct AdhesionLink {
    pub other: Entity,
    pub dir_to_anchor: Vec2,
    pub rest_length: f32,
    pub damping: f32,
    pub breaking_radii: f32,
}

impl AdhesionLink {
    #[must_use]
    pub fn new_from_entity(other: Entity, dir_to_anchor: Vec2) -> Self {
        Self {
            other,
            dir_to_anchor: dir_to_anchor.normalize(),
            rest_length: 1.0, // In radii
            damping: 0.9,
            breaking_radii: 0.75, // In combined radii, how far apart can the adhesion anchors be before the link breaks
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
    const SOLVER_ITERATIONS: usize = 5;
    const MAX_CORRECTION_PER_ITERATION: f32 = 15. / (SOLVER_ITERATIONS as f32);

    // Collect adhesed cells into a vec so that mutable access can be given to the adhesion query within the loop
    let adhesions_vec = adhesions
        .iter()
        .map(|(entity, _, _, adhesion)| (entity, adhesion.clone()))
        .collect::<Vec<_>>();

    for _ in 0..SOLVER_ITERATIONS {
        for (entity_1, adhesion_1) in &adhesions_vec {
            for link in &adhesion_1.links {
                if let Ok(
                [
                    (_, cell_1, mut transform_1, _),
                    (entity_2, cell_2, mut transform_2, _adhesion_2),
                ],
            ) = adhesions.get_many_mut([*entity_1, link.other])
                // Enforce an ordering so adhesion resolution isn't repeated
                && *entity_1 >= entity_2
                {
                    // Calculate the direction to cell 2 from cell 1 in world coords
                    let world_dir_to_cell_2 = transform_1.rotation.mul_vec3(link.dir_to_anchor.extend(0.)).xy().normalize();

                    let cell_1_radius = cell_1.get_size(&param, &game_mode).x * 0.5;
                    let cell_2_radius = cell_2.get_size(&param, &game_mode).x * 0.5;

                    // Calculate the mass ratios of each cell, to be used to scale the correction
                    let cell_1_mass = cell_1.get_mass(&param);
                    let cell_2_mass = cell_2.get_mass(&param);
                    let cell_1_mass_ratio = cell_2_mass / (cell_1_mass + cell_2_mass);
                    let cell_2_mass_ratio = cell_1_mass / (cell_1_mass + cell_2_mass);

                    // Calculate the positions of the anchors on each cell
                    let world_anchor_1 = transform_1.translation.xy() + world_dir_to_cell_2 * cell_1_radius;
                    let world_anchor_2 = transform_2.translation.xy() - world_dir_to_cell_2 * cell_2_radius;

                    // Distance betwen anchors
                    let anchor_delta = world_anchor_2 - world_anchor_1;

                    // Damp the correction, and clamp its maximum value
                    let total_correction = (anchor_delta * (link.damping / (SOLVER_ITERATIONS as f32)))
                        .clamp_length_max(MAX_CORRECTION_PER_ITERATION);

                    // Scale correction by the mass ratios then apply it
                    transform_1.translation += (total_correction * cell_1_mass_ratio).extend(0.);
                    transform_2.translation -= (total_correction * cell_2_mass_ratio).extend(0.);
                }
            }
        }
    }
}

pub fn adhesion_cleanup(mut commands: Commands, adhesion_query: Query<(Entity, &Adhesion), With<PendingDespawn>>) {
    for (entity, adhesions) in &adhesion_query {
        // Remove this cell from neighbours’ link lists
        for neighbour in &adhesions.links {
            if let Ok((_, neighbour_adhesion)) = adhesion_query.get(neighbour.other) {
                commands.entity(neighbour.other).insert(Adhesion {
                    links: neighbour_adhesion
                        .links
                        .iter()
                        .filter(|x| x.other != entity)
                        .cloned()
                        .collect(),
                });
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn break_long_adhesions(
    mut adhesion_query: Query<(Entity, &Cell, &Transform, &mut Adhesion), Without<PendingDespawn>>,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
) {
    // Collect adhesed cells into a vec so that mutable access can be given to the adhesion query within the loop
    let adhesions_vec = adhesion_query
        .iter()
        .map(|(entity, _, _, adhesion)| (entity, adhesion.clone()))
        .collect::<Vec<_>>();

    for (entity, adhesion) in &adhesions_vec {
        for link in &adhesion.links {
            // Specify an ordering so that this code isn't repeated on the same cells
            if *entity >= link.other {
                // Get mutable access to this entity's, and the other entity's, adhesion
                if let Ok(
                    [
                        (_, cell, transform, mut adhesion),
                        (other_entity, other_cell, other_transform, mut other_adhesion),
                    ],
                ) = adhesion_query.get_many_mut([*entity, link.other])
                {
                    // Convert the local dir to other into world coords
                    let world_dir_to_other = link
                        .dir_to_anchor
                        .rotate(Vec2::from_angle(transform.rotation.to_euler(EulerRot::XYZ).2))
                        .normalize();

                    let cell_radius = cell.get_size(&param, &game_mode).x * 0.5;
                    let other_cell_radius = other_cell.get_size(&param, &game_mode).x * 0.5;

                    let anchor_1 = transform.translation.xy() + world_dir_to_other * cell_radius;
                    let anchor_2 = other_transform.translation.xy() - world_dir_to_other * other_cell_radius;

                    // Check if the adhesion anchors are too far apart
                    if (anchor_2 - anchor_1).length() >= (cell_radius + other_cell_radius) * link.breaking_radii {
                        // Remove the links pointing to other entity from the current entity's adhesion component
                        adhesion.links.retain(|link| link.other != other_entity);

                        // Remove the links pointing to current entity from the other entity's adhesion component
                        other_adhesion.links.retain(|link| link.other != *entity);
                    }
                }
            }
        }
    }
}

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

                let world_anchor_1 = transform.translation.xy() + world_dir * cell_1_radius * 0.7;
                let world_anchor_2 = other_transform.translation.xy() - world_dir * cell_2_radius * 0.7;

                gizmos.line(
                    world_anchor_1.extend(4.),
                    world_anchor_2.extend(4.),
                    Color::linear_rgb(0., 0., 1.),
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn add_adhesion_to_daughters(
    commands: &mut Commands,

    parent_entity: Entity,
    parent_transform: &Transform,
    parent_adhesion: &Option<Adhesion>,
    parent_genome_mode: &GenomeMode,

    d1_entity: Entity,
    d1_bundle: &CellBundle,

    d2_entity: Entity,
    d2_bundle: &CellBundle,

    cells: &mut Query<(Entity, &Cell, &Transform, &Velocity, Option<&mut Adhesion>), Without<PendingDespawn>>,
) {
    let (mut links_1, mut links_2) = (vec![], vec![]);

    // Get the split angle as a vector
    let parent_split_axis =
        Vec2::from_angle(parent_genome_mode.split_angle + parent_transform.rotation.to_euler(EulerRot::XYZ).2).normalize();

    // If the parent had adhesion already applied to it // TODO Check if the daughter has keep adhesion
    if let Some(adhesion) = parent_adhesion {
        const ADHESION_ANGLE_DELTA: f32 = 10f32.to_radians(); // The +/- angle that dictates which angles to classify as perpendicular or not when adhesing

        for link in &adhesion.links {
            // Convert local direction to world direction from linked cell (to parent)
            let world_dir_from_other = link
                .dir_to_anchor
                .rotate(Vec2::from_angle(parent_transform.rotation.to_euler(EulerRot::XYZ).2))
                .normalize();

            // Get mutable access to the other cell's adhesion so its links can be modified
            if let Ok((_, _, _, _, other_adhesion)) = cells.get_mut(link.other)
                && let Some(mut other_adhesion) = other_adhesion
            {
                // Find the link in 'other' which points to the parent entity
                if let Some(other_link) = other_adhesion.links.iter_mut().find(|l| l.other == parent_entity) {
                    // Both daugthers can be connected if the link direction is almost perpendicular to the split axis
                    if (world_dir_from_other.dot(parent_split_axis)).abs() <= 1.0 - ADHESION_ANGLE_DELTA.cos().abs() {
                        // TODO Create connections for both daughters
                        println!("Both daughters need to connect");
                    }
                    // Only one daugther can be connected if the link direction is not almost perpendicular to the split axis
                    else {
                        // Use daughter 1 if split axis and word_dir_from_other are negatively aligned
                        if parent_split_axis.dot(world_dir_from_other).is_sign_negative() {
                            // Calculate the direction to the other entity in local coords for daughter 1
                            let local_dir = d1_bundle
                                .transform
                                .rotation
                                .inverse()
                                .mul_vec3(world_dir_from_other.extend(0.))
                                .normalize()
                                .xy();

                            // Then add this link to the vec
                            links_1.push(AdhesionLink::new_from_entity(link.other, local_dir));

                            // Replace the other_link's entity with this daughter's entity
                            other_link.other = d1_entity;
                        }
                        // Use daughter 2 if split axis and word_dir_from_other are positively aligned
                        else {
                            // Calculate the direction to the other entity in local coords for daughter 2
                            let local_dir = d2_bundle
                                .transform
                                .rotation
                                .inverse()
                                .mul_vec3(world_dir_from_other.extend(0.))
                                .normalize()
                                .xy();

                            // Then add this link to the vec
                            links_2.push(AdhesionLink::new_from_entity(link.other, local_dir));

                            // Replace the other_link's entity with this daughter's entity
                            other_link.other = d2_entity;
                        }
                    }
                }
            }
        }
    }

    // Create an adhesion link between the two daughters

    // Calculate the direction from daughter 1 to daughter 2 in world coords
    let world_dir = (d2_bundle.transform.translation.xy() - d1_bundle.transform.translation.xy()).normalize();

    // Calculate the local directions to each daughter in their local coords
    let local_dir_1 = d1_bundle
        .transform
        .rotation
        .inverse()
        .mul_vec3(world_dir.extend(0.))
        .normalize()
        .xy();
    let local_dir_2 = d2_bundle
        .transform
        .rotation
        .inverse()
        .mul_vec3(-world_dir.extend(0.))
        .normalize()
        .xy();

    // Add the links
    links_1.push(AdhesionLink::new_from_entity(d2_entity, local_dir_1));
    links_2.push(AdhesionLink::new_from_entity(d1_entity, local_dir_2));

    // Insert the adhesion components for each daughter, with the specified links
    commands.entity(d1_entity).insert(Adhesion { links: links_1 });
    commands.entity(d2_entity).insert(Adhesion { links: links_2 });
}
