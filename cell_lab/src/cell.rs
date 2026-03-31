use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use rand::RngExt;

use std::f32::consts::PI;

use crate::{
    cell_material::CellMaterial,
    chemical::Chemical,
    genome::{Genome, GenomeId},
    genome_bank::{GenomeBankId, GenomeCollection},
    state::PlayModeState,
};

#[derive(Component)]
pub struct Velocity(pub Vec2);

// Cell parameters
pub const CELL_ENERGY: f32 = 40.;
pub const CELL_MAX_VELOCITY: f32 = 100.;
const RANDOM_ACCELERATION: f32 = 10.;
pub const STARTING_CELL_NUM: u32 = 20;
const CELL_DIVISION_ENERGY: f32 = 60.;
const CELL_ENERGY_DECAY: f32 = 1.;
pub const MAX_CELL_AGE: f32 = 100.;
pub const MAX_CELL_ENERGY: f32 = 100.;
pub const MIN_CELL_ENERGY: f32 = 1.;

#[derive(Component, Debug, Clone)]
pub struct Cell {
    pub energy: f32,
    pub age: f32,
    pub genome_id: GenomeId,
    pub genome_bank_id: GenomeBankId,
}

impl Cell {
    // #[must_use]
    // pub fn new_bundle(
    //     // genome_bank: GenomeBank,
    //     energy: f32,
    //     velocity: Vec2,
    //     position: Vec2,
    //     colour: Color,
    //     meshes: &mut ResMut<Assets<Mesh>>,
    //     materials: &mut ResMut<Assets<CellMaterial>>,
    // ) -> impl Bundle {
    //     let cell = Self {
    //         energy,
    //         age: 0.,
    //         genome_id: GenomeId::default(),
    //     };
    //     (
    //         cell.clone(),
    //         Velocity(velocity),
    //         Transform::from_translation(position.extend(1.)).with_scale(cell.get_size().extend(1.)),
    //         Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
    //         // MeshMaterial2d(materials.add(CellMaterial::new(genome_bank[cell.genome_id].colour))),
    //         MeshMaterial2d(materials.add(CellMaterial::new(colour))),
    //     )
    // }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_genome(
        energy: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        genome_collection: &GenomeCollection,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CellMaterial>>,
    ) -> impl Bundle {
        let cell = Self {
            energy,
            age: 0.,
            genome_id,
            genome_bank_id,
        };
        (
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size().extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome(genome_collection).colour))),
        )
    }

    //TODO
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_genome_and_age(
        energy: f32,
        age: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        genome_collection: &GenomeCollection,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CellMaterial>>,
    ) -> impl Bundle {
        let cell = Self {
            energy,
            age,
            genome_id,
            genome_bank_id,
        };
        (
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size().extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome(genome_collection).colour))),
        )
    }

    #[must_use]
    pub fn get_genome<'a>(&self, genome_collection: &'a GenomeCollection) -> &'a Genome {
        &genome_collection[self.genome_bank_id][self.genome_id]
    }

    #[must_use]
    pub fn get_size(&self) -> Vec2 {
        Vec2::splat(self.energy * 2.)
    }
}

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
    mut query: Query<(&mut Transform, &mut Cell, &mut Velocity)>,
) {
    // TODO Reuse splitting from editor

    for (mut transform, mut cell, mut velocity) in &mut query {
        if cell.energy > CELL_DIVISION_ENERGY {
            // Generate a random angle for the velocity
            let angle = rand::rng().random::<f32>() * PI;

            // Rotate the velocity to match these angles
            let v1 = velocity.0.rotate(Vec2::from_angle(angle / 2.));
            let v2 = velocity.0.rotate(Vec2::from_angle(-angle / 2.));

            // Scale the magnitude so it conserves momentum
            let magnitude_scale = velocity.0.length() / (v1 + v2).length();

            // Create a new cell
            commands.spawn(Cell::new_bundle_with_genome(
                cell.energy / 2.,
                cell.genome_id,
                cell.genome_bank_id,
                v2 * magnitude_scale,
                transform.translation.xy(),
                &genome_collection,
                &mut meshes,
                &mut materials,
            ));

            // Change cell energy and velocity, then resize cell
            cell.energy /= 2.;
            velocity.0 = v1 * magnitude_scale;
            transform.scale = cell.get_size().extend(1.);
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
