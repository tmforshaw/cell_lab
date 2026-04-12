use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{
        Cell, CellMaterial, Velocity,
        adhesion::{Adhesion, AdhesionLink},
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

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::too_many_lines
)]
pub fn cells_do_meiosis(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    genome_bank: Res<GenomeBank>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    mut cells: Query<(Entity, &Cell, &Transform, &Velocity, Option<&mut Adhesion>), Without<PendingDespawn>>,
) {
    enum LinkOp {
        Replace {
            other: Entity,
            old: Entity,
            new: Entity,
        },
        Split {
            other: Entity,
            old: Entity,
            new_1: Entity,
            new_dir_1: Vec2,
            new_2: Entity,
            new_dir_2: Vec2,
        },
    }

    let mut link_ops: Vec<LinkOp> = vec![];

    for (parent_entity, parent, transform, velocity, adhesion) in &cells {
        if let Some((d1_bundle, d2_bundle)) = parent.split_into_daughter_bundles(
            &genome_bank,
            transform,
            velocity,
            &param,
            &game_mode,
            &mut meshes,
            &mut materials,
        ) {
            let parent_genome = parent.get_genome_mode(&genome_bank);

            // Spawn the daughters
            let d1_entity = commands.spawn(d1_bundle.clone()).id();
            let d2_entity = commands.spawn(d2_bundle.clone()).id();

            // Add adhesion if parent says it's neccessary
            if parent_genome.daughters_adhere {
                let (mut links_1, mut links_2) = (vec![], vec![]);

                // Get the split angle as a vector
                let parent_split_dir =
                    Vec2::from_angle(parent_genome.split_angle + transform.rotation.to_euler(EulerRot::XYZ).2).normalize();

                // If the parent had adhesion already applied to it
                if let Some(adhesion) = adhesion {
                    const ADHESION_ANGLE_DELTA: f32 = 10f32.to_radians(); // The +/- angle that dictates which angles to classify as perpendicular or not when adhesing

                    for link in &adhesion.links {
                        // Convert local direction to world direction to other in link
                        let world_dir = link
                            .dir_to_anchor
                            .rotate(Vec2::from_angle(transform.rotation.to_euler(EulerRot::XYZ).2))
                            .normalize();

                        if let Ok((_, other, other_transform, _other_velocity, _other_adhesion)) = cells.get(link.other) {
                            let other_pos = other_transform.translation.xy();
                            let other_size = other.get_size(&param, &game_mode).x * 0.5;

                            // Both daugthers can be connected if the link direction is almost perpendicular to the split direction
                            if (world_dir.dot(parent_split_dir)).abs() <= 1.0 - ADHESION_ANGLE_DELTA.cos().abs() {
                                // TODO Create connections for both daughters
                                println!("Both daughters need to connect");

                                // // Add the new daughter links to the other cell, removing the old link to the parent
                                // link_ops.push(LinkOp::Split{
                                //     other: link.other,
                                //     old: parent_entity,
                                //     new_1: d1_entity,
                                //     new_dir_1: todo!(),
                                //     new_2: d2_entity,
                                //     new_dir_2: todo!(),
                                // });
                            }
                            // Only one daugther can be connected if the link direction is not almost perpendicular to the split direction
                            else {
                                // Calculate where the anchor is for the linked cell
                                let anchor_on_other = other_pos + world_dir * other_size;

                                let d1_dist_to_surface = d1_bundle
                                    .cell
                                    .get_size(&param, &game_mode)
                                    .x
                                    .mul_add(-0.5, (d1_bundle.transform.translation.xy() - anchor_on_other).length());
                                let d2_dist_to_surface = d2_bundle
                                    .cell
                                    .get_size(&param, &game_mode)
                                    .x
                                    .mul_add(-0.5, (d2_bundle.transform.translation.xy() - anchor_on_other).length());

                                // TODO Don't do it from distance, do it based on (Daughter1 is left Daughter2 is right)
                                // Test which daughter is closest to this contact point
                                if d1_dist_to_surface < d2_dist_to_surface {
                                    // Calculate the direction to the other entity in local coords for daughter 1
                                    let local_dir = d1_bundle
                                        .transform
                                        .rotation
                                        .inverse()
                                        .mul_vec3(-world_dir.extend(0.))
                                        .normalize()
                                        .xy();

                                    // Then add this link to the vec
                                    links_1.push(AdhesionLink::new_from_entity(link.other, local_dir));

                                    // Replace this link with the daughter instead of the parent
                                    link_ops.push(LinkOp::Replace {
                                        other: link.other,
                                        old: parent_entity,
                                        new: d1_entity,
                                    });
                                } else {
                                    // Calculate the direction to the other entity in local coords for daughter 2
                                    let local_dir = d2_bundle
                                        .transform
                                        .rotation
                                        .inverse()
                                        .mul_vec3(-world_dir.extend(0.))
                                        .normalize()
                                        .xy();

                                    // Then add this link to the vec
                                    links_2.push(AdhesionLink::new_from_entity(link.other, local_dir));

                                    // Replace this link with the daughter instead of the parent
                                    link_ops.push(LinkOp::Replace {
                                        other: link.other,
                                        old: parent_entity,
                                        new: d2_entity,
                                    });
                                }
                            }
                        }
                    }
                }

                // Create an adhesion link between the two daughters

                // Calculate the direction between the daughters in world coords
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

            // Despawn the parent cell
            commands.entity(parent_entity).insert(PendingDespawn);
        } else {
            // Didn't split
        }
    }

    // Apply the link operations
    for link_op in link_ops {
        match link_op {
            LinkOp::Replace { other, old, new } => {
                // Get the other cell, and its adhesion component
                if let Ok((_, _, _, _, adhesion)) = cells.get_mut(other)
                    && let Some(mut adhesion) = adhesion
                {
                    // Find the link which points to the old entity (To be replaced)
                    if let Some(link) = adhesion.links.iter_mut().find(|l| l.other == old) {
                        // Replace the entity which is referenced to the new entity
                        link.other = new;
                    }
                }
            }
            LinkOp::Split {
                other,
                old,
                new_1,
                new_dir_1,
                new_2,
                new_dir_2,
            } => todo!(),
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
