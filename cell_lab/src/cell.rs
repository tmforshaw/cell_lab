use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use rand::RngExt;

use crate::{
    cell_material::CellMaterial,
    chemical::Chemical,
    genome::{CellSplitType, Genome, GenomeId, get_daughter_data},
    genome_bank::{GenomeBankId, GenomeCollection},
    state::PlayModeState,
};

#[derive(Component)]
pub struct Velocity(pub Vec2);

// Cell parameters
pub const CELL_ENERGY: f32 = 10.;
pub const CELL_MAX_VELOCITY: f32 = 100.;
const RANDOM_ACCELERATION: f32 = 10.;
pub const STARTING_CELL_NUM: u32 = 20;
const CELL_ENERGY_DECAY: f32 = 1.;
pub const MAX_CELL_AGE: f32 = 100.;
pub const MAX_CELL_ENERGY: f32 = 100.;
pub const MIN_CELL_ENERGY: f32 = 1.;
pub const CELL_SPLIT_PADDING: f32 = 1.2; // Multiplier for offset of daughters from each other (Multiplies radius)

#[derive(Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub velocity: Velocity,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<CellMaterial>,
}

impl CellBundle {
    #[must_use]
    pub const fn new(
        cell: Cell,
        velocity: Velocity,
        transform: Transform,
        mesh: Mesh2d,
        material: MeshMaterial2d<CellMaterial>,
    ) -> Self {
        Self {
            cell,
            velocity,
            transform,
            mesh,
            material,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Cell {
    pub energy: f32,
    pub age: f32,
    pub genome_id: GenomeId,
    pub genome_bank_id: GenomeBankId,
}

impl Cell {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle(
        energy: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        genome_collection: &GenomeCollection,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age: 0.,
            genome_id,
            genome_bank_id,
        };
        CellBundle::new(
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
    pub fn new_bundle_with_age(
        energy: f32,
        age: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        genome_collection: &GenomeCollection,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age,
            genome_id,
            genome_bank_id,
        };
        CellBundle::new(
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

    pub fn split_into_daughter_bundles(
        &self,
        genome_collection: &GenomeCollection,
        transform: &Transform,
        velocity: &Velocity,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CellMaterial>>,
    ) -> Option<(CellBundle, CellBundle)> {
        self.split_into_daughter_bundles_with_age(0.0, genome_collection, transform, velocity, meshes, materials)
    }

    // TODO Add parent's velocity into this to conserve momentum
    pub fn split_into_daughter_bundles_with_age(
        &self,
        age: f32,
        genome_collection: &GenomeCollection,
        transform: &Transform,
        velocity: &Velocity,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> Option<(CellBundle, CellBundle)> {
        let genome = self.get_genome(genome_collection);

        match genome.split_type {
            CellSplitType::Age | CellSplitType::Energy => {
                // Check the correct parameter based on the parent's split type
                if (genome.split_type == CellSplitType::Age && self.age >= genome.split_age)
                    || (genome.split_type == CellSplitType::Energy && self.energy >= genome.split_energy)
                {
                    // Get the data for both daughters
                    let (d1, d2) = get_daughter_data(
                        self,
                        transform.translation.xy(),
                        velocity.0,
                        transform.scale.xy(),
                        genome_collection,
                    );

                    return Some((
                        // Set the first daughter's parameters, and get its bundle
                        Self::new_bundle_with_age(
                            d1.energy,
                            age,
                            d1.genome_id,
                            self.genome_bank_id,
                            d1.velocity,
                            d1.position,
                            genome_collection,
                            meshes,
                            materials,
                        ),
                        // Set the second daughter's parameters, and get its bundle
                        Self::new_bundle_with_age(
                            d2.energy,
                            age,
                            d2.genome_id,
                            self.genome_bank_id,
                            d2.velocity,
                            d2.position,
                            genome_collection,
                            meshes,
                            materials,
                        ),
                    ));
                }
            }
            CellSplitType::Never => {}
        }

        None
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
