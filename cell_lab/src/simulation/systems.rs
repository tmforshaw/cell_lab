use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{
        Cell, CellMaterial, Velocity,
        adhesion::{AdhesionParameters, Adhesions},
    },
    despawning::PendingDespawn,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::GenomeBank,
    helpers::random_vec2,
    simulation::chemical::{Chemical, ChemicalMaterial, ChemicalTimer},
    spatial_partitioning::chemical_quadtree::ChemicalQuadTree,
};

// Make cells age up
#[allow(clippy::needless_pass_by_value)]
pub fn increment_cell_age(time: Res<Time>, mut query: Query<&mut Cell, Without<PendingDespawn>>) {
    let dt = time.delta_secs();
    for mut cell in &mut query {
        cell.age += dt;
    }
}

// Move cells smoothly
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn move_cells(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<Cell>, Without<PendingDespawn>)>,
    param: Res<GameParameters>,
) {
    let dt = time.delta().as_secs_f32();

    let max_velocity = param.cell_parameters.max_velocity;
    for (mut transform, mut velocity) in &mut query {
        // Clamp speed
        velocity.0 = velocity.0.clamp(Vec2::splat(-max_velocity), Vec2::splat(max_velocity));

        // Move
        transform.translation += (velocity.0 * dt).extend(0.);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn bound_cells(
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<Cell>, Without<PendingDespawn>)>,
) {
    for (mut transform, mut velocity) in &mut query {
        let size = transform.scale.xy();

        let dish_size = match game_mode.get() {
            GameMode::Simulation => param.simulation_mode.dish_parameters.size,
            GameMode::CellEditor => param.cell_editor_mode.dish_parameters.size,
        };

        let bounds = (dish_size - size) / 2.;

        // X Bound Collision Resolution
        if transform.translation.x <= -bounds.x {
            velocity.0.x *= -1.;
            transform.translation.x = -bounds.x;
        } else if transform.translation.x >= bounds.x {
            velocity.0.x *= -1.;
            transform.translation.x = bounds.x;
        }

        // Y Bound Collision Resolution
        if transform.translation.y <= -bounds.y {
            velocity.0.y *= -1.;
            transform.translation.y = -bounds.y;
        } else if transform.translation.y >= bounds.y {
            velocity.0.y *= -1.;
            transform.translation.y = bounds.y;
        }
    }
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn cells_absorb_chemical(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    chemical_quadtree: Res<ChemicalQuadTree>,
    mut cell_query: Query<(&mut Cell, &mut Transform), (Without<Chemical>, Without<PendingDespawn>)>,
    chemicals: Query<(Entity, &Chemical, &Transform), (Without<Cell>, Without<PendingDespawn>)>,
) {
    // Assume that quadtrees are already built, so just get roots
    let chemical_quadtree_root = chemical_quadtree.0.get_root();

    for (mut cell, mut cell_transform) in &mut cell_query {
        // Only absorb chemicals if cell has space for it
        if cell.energy < param.cell_parameters.max_energy {
            // Don't half the size, to include neighbouring quadrants
            let cell_aabb = Aabb2d::new(cell_transform.translation.xy(), cell_transform.scale.xy());

            let mut candidates = Vec::new();
            chemical_quadtree_root.query(&cell_aabb, &mut candidates);

            // Iterate through coarse collision detection candidates
            for chemical_entity in candidates {
                // Get the chemical's data
                if let Ok((_chemical_entity, chemical, chemical_transform)) = chemicals.get(chemical_entity) {
                    let dist = (cell_transform.translation - chemical_transform.translation).length();
                    let combined_radius = f32::midpoint(cell_transform.scale.x, chemical_transform.scale.x); // Use X since X and Y are identical

                    // Collision has occurred
                    if dist < combined_radius {
                        // Cell gains the chemical's energy then resizes based on new energy
                        cell.energy += (chemical.energy).min(param.cell_parameters.max_energy - cell.energy);
                        cell_transform.scale = cell.get_size(&param, &game_mode).extend(1.);

                        // Despawn the chemical
                        commands.entity(chemical_entity).insert(PendingDespawn);
                    }
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)]
pub fn cells_do_meiosis(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    genome_bank: Res<GenomeBank>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    cells: Query<(Entity, &mut Cell, &Transform, &Velocity, Option<&Adhesions>), Without<PendingDespawn>>,
    adhesion_query: Query<(&Adhesions, &Transform), Without<PendingDespawn>>,
) {
    for (parent_entity, parent, transform, velocity, parent_adhesion) in cells {
        if let Some((d1_bundle, d2_bundle)) = parent.split_into_daughter_bundles(
            &genome_bank,
            transform,
            velocity,
            &param,
            &game_mode,
            &mut meshes,
            &mut materials,
        ) {
            // Spawn the daughters
            let d1_entity = commands.spawn(d1_bundle.clone()).id();
            let d2_entity = commands.spawn(d2_bundle.clone()).id();

            // Add adhesion if parent says it's neccessary
            if parent.get_genome_mode(&genome_bank).daughters_adhere {
                const MAX_CONNECTIONS: usize = 3; // TODO

                // New links for daughters
                let mut d1_links = vec![d2_entity];
                let mut d2_links = vec![d1_entity];

                // If the parent was already adhered to some cells
                if let Some(parent_adhesion) = parent_adhesion {
                    // Link daughters to parent’s neighbours
                    for &neighbour in &parent_adhesion.links {
                        // Skip the parent itself
                        if neighbour == parent_entity {
                            continue;
                        }

                        // Check if neighbour is still valid
                        if let Ok((neighbour_adhesion, neighbour_transform)) = adhesion_query.get(neighbour) {
                            // Ensure neighbour still has parent link
                            if neighbour_adhesion.links.contains(&parent_entity) {
                                fn connect_to_parent_connections(
                                    commands: &mut Commands,
                                    neighbour: Entity,
                                    neighbour_adhesion: &Adhesions,
                                    parent_entity: Entity,
                                    daughter_entity: Entity,
                                ) {
                                    // Update neighbour to link to daughters instead of parent
                                    commands.entity(neighbour).insert(Adhesions {
                                        links: neighbour_adhesion
                                            .links
                                            .iter()
                                            .map(|&neighbour_link| {
                                                if neighbour_link == parent_entity {
                                                    daughter_entity
                                                } else {
                                                    neighbour_link
                                                }
                                            })
                                            .collect(),
                                        params: neighbour_adhesion.params.clone(),
                                    });
                                }

                                let neighbour_pos = neighbour_transform.translation.xy();

                                let d1_touching = (d1_bundle.transform.translation.xy() - neighbour_pos).length()
                                    < d1_bundle.cell.get_size(&param, &game_mode).x * 1.5; // TODO use neighbour size
                                let d2_touching = (d2_bundle.transform.translation.xy() - neighbour_pos).length()
                                    < d2_bundle.cell.get_size(&param, &game_mode).x * 1.5;

                                // Link neighbour to daughters if the daughter is touching that neighbour
                                if d1_touching || d2_touching {
                                    if d1_touching && d1_links.len() < MAX_CONNECTIONS {
                                        d1_links.push(neighbour);

                                        // Update neighbour to link to daughters instead of parent
                                        connect_to_parent_connections(
                                            &mut commands,
                                            neighbour,
                                            neighbour_adhesion,
                                            parent_entity,
                                            d1_entity,
                                        );
                                    }
                                    if d2_touching && d2_links.len() < MAX_CONNECTIONS {
                                        d2_links.push(neighbour);

                                        // Update neighbour to link to daughters instead of parent
                                        connect_to_parent_connections(
                                            &mut commands,
                                            neighbour,
                                            neighbour_adhesion,
                                            parent_entity,
                                            d2_entity,
                                        );
                                    }
                                }
                            }
                        }
                    }

                    // Insert adhesion components for daughters
                    commands.entity(d1_entity).insert(Adhesions {
                        links: d1_links,
                        params: parent_adhesion.params.clone(),
                    });

                    commands.entity(d2_entity).insert(Adhesions {
                        links: d2_links,
                        params: parent_adhesion.params.clone(),
                    });
                } else {
                    // TODO Remove this
                    let adhesion_params = AdhesionParameters::default();

                    // Spawn the daughters with default links
                    commands.entity(d1_entity).insert(Adhesions {
                        links: d1_links,
                        params: adhesion_params.clone(),
                    });

                    commands.entity(d2_entity).insert(Adhesions {
                        links: d2_links,
                        params: adhesion_params,
                    });
                }
            }

            // Despawn the parent cell
            commands.entity(parent_entity).insert(PendingDespawn);
        } else {
            // Didn't split
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_decay(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Cell), Without<PendingDespawn>>,
) {
    let dt = time.delta().as_secs_f32();

    for (entity, mut transform, mut cell) in &mut query {
        // Reduce energy
        cell.energy -= param.cell_energy_decay_rate * dt;

        // Remove cell if its too small
        if cell.energy <= param.cell_parameters.min_energy {
            commands.entity(entity).insert(PendingDespawn);
        } else {
            // Resize the cell
            transform.scale = cell.get_size(&param, &game_mode).extend(1.);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_chemicals(
    mut commands: Commands,
    param: Res<GameParameters>,
    mut materials: ResMut<Assets<ChemicalMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut timer: ResMut<ChemicalTimer>,
    chemicals: Query<(), (With<Chemical>, Without<PendingDespawn>)>,
) {
    timer.0.tick(time.delta());

    if chemicals.count() < param.simulation_mode.chemical_parameters.max_instances {
        // Spawn a random chemical depending on the spawn rate
        if timer.0.just_finished() {
            let chemical_bounds =
                (param.simulation_mode.dish_parameters.size - param.simulation_mode.chemical_parameters.size) / 2.;

            let random_pos = random_vec2(chemical_bounds);

            commands.spawn((
                Chemical {
                    energy: param.simulation_mode.chemical_parameters.energy,
                },
                Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                MeshMaterial2d(materials.add(ChemicalMaterial::new(param.simulation_mode.chemical_parameters.colour))),
                Transform::from_xyz(random_pos.x, random_pos.y, 0.5)
                    .with_scale(Vec2::splat(param.simulation_mode.chemical_parameters.size).extend(1.)),
            ));
        }
    }
}
