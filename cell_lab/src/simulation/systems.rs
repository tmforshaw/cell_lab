use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use rand::RngExt;

use crate::{
    cells::{CELL_ENERGY_DECAY, CELL_MAX_VELOCITY, Cell, CellMaterial, MIN_CELL_ENERGY, RANDOM_ACCELERATION, Velocity},
    genomes::GenomeCollection,
    helpers::random_vec2,
    simulation::chemical::{
        CHEMICAL_COLOUR, CHEMICAL_ENERGY, CHEMICAL_MAX_NUM, CHEMICAL_SIZE, Chemical, ChemicalMaterial, ChemicalTimer,
    },
    state::PlayModeState,
};

// Make cells age up
#[allow(clippy::needless_pass_by_value)]
pub fn increment_cell_age(time: Res<Time>, mut query: Query<&mut Cell>) {
    let dt = time.delta_secs();
    for mut cell in &mut query {
        cell.age += dt;
    }
}

// Move cells smoothly
#[allow(clippy::needless_pass_by_value)]
pub fn move_cells(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity), With<Cell>>) {
    let dt = time.delta().as_secs_f32();
    let mut rng = rand::rng();

    for (mut transform, mut velocity) in &mut query {
        // Slight random acceleration
        velocity.0 += Vec2::new(
            rng.random_range(-RANDOM_ACCELERATION..RANDOM_ACCELERATION),
            rng.random_range(-RANDOM_ACCELERATION..RANDOM_ACCELERATION),
        ) * dt;

        // Clamp speed
        velocity.0 = velocity
            .0
            .clamp(Vec2::splat(-CELL_MAX_VELOCITY), Vec2::splat(CELL_MAX_VELOCITY));

        // Move
        transform.translation += (velocity.0 * dt).extend(0.);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn bound_cells(state: Res<PlayModeState>, mut query: Query<(&mut Transform, &mut Velocity), With<Cell>>) {
    for (mut transform, mut velocity) in &mut query {
        let size = transform.scale.xy();

        let bounds = (state.dish.size - size) / 2.;

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

pub fn cells_absorb_chemical(
    mut commands: Commands,
    mut cell_query: Query<(&mut Transform, &mut Cell), Without<Chemical>>,
    chemical_query: Query<(&Transform, &Chemical, Entity), Without<Cell>>,
) {
    for (mut cell_transform, mut cell) in &mut cell_query {
        for (chemical_transform, chemical, chemical_entity) in chemical_query.iter() {
            // They both have sizes defined
            let (cell_size, chemical_size) = (cell_transform.scale.xy(), chemical_transform.scale.xy());

            // Generate bounding boxes
            let cell_aabb = Aabb2d::new(cell_transform.translation.xy(), cell_size / 2.);
            let chemical_aabb = Aabb2d::new(chemical_transform.translation.xy(), chemical_size / 2.);

            // Collision detected
            if cell_aabb.intersects(&chemical_aabb) {
                // Gain energy then resize cell based on new energy
                cell.energy += chemical.energy;
                cell_transform.scale = cell.get_size().extend(1.);

                // Despawn the chemical
                commands.entity(chemical_entity).despawn();
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cells_do_meiosis(
    mut commands: Commands,
    genome_collection: Res<GenomeCollection>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CellMaterial>>,
    cells: Query<(Entity, &mut Cell, &Transform, &Velocity)>,
) {
    for (entity, parent, transform, velocity) in cells {
        if let Some((d1_bundle, d2_bundle)) =
            parent.split_into_daughter_bundles(&genome_collection, transform, velocity, &mut meshes, &mut materials)
        {
            // Spawn the daughters
            commands.spawn(d1_bundle);
            commands.spawn(d2_bundle);

            // Despawn the parent cell
            commands.entity(entity).despawn();
        } else {
            // Didn't split
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn cell_decay(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Transform, &mut Cell, Entity)>) {
    let dt = time.delta().as_secs_f32();

    for (mut transform, mut cell, entity) in &mut query {
        // Reduce energy
        cell.energy -= CELL_ENERGY_DECAY * dt;

        // Remove cell if its too small
        if cell.energy <= MIN_CELL_ENERGY {
            commands.entity(entity).despawn();
        } else {
            // Resize the cell
            transform.scale = cell.get_size().extend(1.);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_chemicals(
    mut commands: Commands,
    mut materials: ResMut<Assets<ChemicalMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    state: Res<PlayModeState>,
    mut timer: ResMut<ChemicalTimer>,
    chemicals: Query<(), With<Chemical>>,
) {
    timer.0.tick(time.delta());

    if chemicals.count() < CHEMICAL_MAX_NUM {
        // Spawn a random chemical depending on the spawn rate
        if timer.0.just_finished() {
            let chemical_bounds = (state.dish.size - CHEMICAL_SIZE) / 2.;

            let random_pos = random_vec2(chemical_bounds);

            commands.spawn((
                Chemical { energy: CHEMICAL_ENERGY },
                Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                MeshMaterial2d(materials.add(ChemicalMaterial::new(CHEMICAL_COLOUR))),
                Transform::from_xyz(random_pos.x, random_pos.y, 0.5).with_scale(Vec2::splat(CHEMICAL_SIZE).extend(1.)),
            ));
        }
    }
}
