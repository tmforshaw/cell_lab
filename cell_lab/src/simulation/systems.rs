use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::{
    cells::{
        Cell, CellMaterial, Velocity,
        adhesion::{Adhesion, AdhesionParameters},
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

#[allow(clippy::needless_pass_by_value)]
pub fn cells_do_meiosis(
    mut commands: Commands,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
    genome_bank: Res<GenomeBank>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    cells: Query<(Entity, &mut Cell, &Transform, &Velocity), Without<PendingDespawn>>,
) {
    for (entity, parent, transform, velocity) in cells {
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
            let d1_entity = commands.spawn(d1_bundle).id();
            let d2_entity = commands.spawn(d2_bundle).id();

            // Add adhesion if parent says to neccessary
            if parent.get_genome_mode(&genome_bank).daughters_adhere {
                let adhesion_param = AdhesionParameters::default();

                let adhesion_1 = Adhesion {
                    other: d2_entity,
                    params: adhesion_param.clone(),
                };

                let adhesion_2 = Adhesion {
                    other: d1_entity,
                    params: adhesion_param,
                };

                // Attach the adhesion components
                commands.entity(d1_entity).insert(adhesion_1);
                commands.entity(d2_entity).insert(adhesion_2);
            }

            // Despawn the parent cell
            commands.entity(entity).insert(PendingDespawn);
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
    mut query: Query<(&mut Transform, &mut Cell, Entity), Without<PendingDespawn>>,
) {
    let dt = time.delta().as_secs_f32();

    for (mut transform, mut cell, entity) in &mut query {
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
