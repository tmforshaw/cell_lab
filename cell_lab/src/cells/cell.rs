use bevy::prelude::*;

use crate::{
    cells::CellMaterial,
    genomes::{CellSplitType, Genome, GenomeBankId, GenomeCollection, GenomeId, get_daughter_data},
};

#[derive(Component)]
pub struct Velocity(pub Vec2);

// Cell parameters
pub const CELL_ENERGY: f32 = 10.;
pub const CELL_MAX_VELOCITY: f32 = 100.;
pub const STARTING_CELL_NUM: u32 = 20;
pub const RANDOM_ACCELERATION: f32 = 10.;
pub const CELL_ENERGY_DECAY: f32 = 1.;
pub const MAX_CELL_SPLIT_AGE: f32 = 25.;
pub const MAX_CELL_ENERGY: f32 = 100.;
pub const MIN_CELL_ENERGY: f32 = 2.;
pub const CELL_SPLIT_PADDING: f32 = 1.2; // Multiplier for offset of daughters from each other (Multiplies radius)
pub const CELL_SIZE_MULTIPLIER: f32 = 10.;
pub const CELL_SIZE_SCALE_FACTOR: f32 = 0.75;

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

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation(
        energy: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        rotation: f32,
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
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size().extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
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

    //TODO
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation_and_age(
        energy: f32,
        age: f32,
        genome_id: GenomeId,
        genome_bank_id: GenomeBankId,
        velocity: Vec2,
        position: Vec2,
        rotation: f32,
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
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size().extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome(genome_collection).colour))),
        )
    }

    #[must_use]
    pub fn get_genome<'a>(&self, genome_collection: &'a GenomeCollection) -> &'a Genome {
        &genome_collection[self.genome_bank_id][self.genome_id]
    }

    #[must_use]
    pub fn get_mass(&self) -> f32 {
        // Scale energy to get mass
        self.energy.powf(CELL_SIZE_SCALE_FACTOR)
    }

    #[must_use]
    pub fn get_size(&self) -> Vec2 {
        // Get masss then multiply that value to get the size
        Vec2::splat(self.get_mass() * CELL_SIZE_MULTIPLIER)
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
                    let (d1, d2) = get_daughter_data(self, velocity, transform, genome_collection);

                    return Some((
                        // Set the first daughter's parameters, and get its bundle
                        Self::new_bundle_with_rotation_and_age(
                            d1.energy,
                            age,
                            d1.genome_id,
                            self.genome_bank_id,
                            d1.velocity,
                            d1.position,
                            d1.rotation + transform.rotation.to_euler(EulerRot::XYZ).2,
                            genome_collection,
                            meshes,
                            materials,
                        ),
                        // Set the second daughter's parameters, and get its bundle
                        Self::new_bundle_with_rotation_and_age(
                            d2.energy,
                            age,
                            d2.genome_id,
                            self.genome_bank_id,
                            d2.velocity,
                            d2.position,
                            d2.rotation + transform.rotation.to_euler(EulerRot::XYZ).2,
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
