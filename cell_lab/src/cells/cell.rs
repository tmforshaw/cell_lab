use bevy::prelude::*;

use crate::{
    cells::CellMaterial,
    genomes::{CellSplitType, DaughterData, GenomeBank, GenomeId, GenomeMode, GenomeModeId},
    spatial_partitioning::quadtree::QuadTreeData,
};

#[derive(Component, Debug, Clone)]
pub struct Velocity(pub Vec2);

// Cell parameters
pub const CELL_STARTING_ENERGY: f32 = 10.;
pub const CELL_MAX_VELOCITY: f32 = 100.;
pub const STARTING_CELL_NUM: u32 = 100;
pub const CELL_ENERGY_DECAY: f32 = 1.;
pub const CELL_MAX_SPLIT_AGE: f32 = 25.;
pub const CELL_MAX_ENERGY: f32 = 50.;
pub const CELL_MIN_ENERGY: f32 = 2.;
pub const CELL_SPLIT_PADDING: f32 = 1.1; // Multiplier for offset of daughters from each other (Multiplies radius)
pub const CELL_SIZE_SCALE_FACTOR: f32 = 0.5; // The power which determines the scale between energy and mass

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
    pub genome_mode_id: GenomeModeId,
    pub genome_id: GenomeId,
    pub size_per_mass: f32,
}

impl Cell {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle(
        energy: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        size_per_mass: f32,
        velocity: Vec2,
        position: Vec2,
        genome_bank: &GenomeBank,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age: 0.,
            genome_mode_id,
            genome_id,
            size_per_mass,
        };
        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size().extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome_mode(genome_bank).colour))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation(
        energy: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        size_per_mass: f32,
        velocity: Vec2,
        position: Vec2,
        rotation: f32,
        genome_bank: &GenomeBank,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age: 0.,
            genome_mode_id,
            genome_id,
            size_per_mass,
        };
        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size().extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome_mode(genome_bank).colour))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_age(
        energy: f32,
        age: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        size_per_mass: f32,
        velocity: Vec2,
        position: Vec2,
        genome_bank: &GenomeBank,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age,
            genome_mode_id,
            genome_id,
            size_per_mass,
        };
        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size().extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome_mode(genome_bank).colour))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation_and_age(
        energy: f32,
        age: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        size_per_mass: f32,
        velocity: Vec2,
        position: Vec2,
        rotation: f32,
        genome_bank: &GenomeBank,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<CellMaterial>,
    ) -> CellBundle {
        let cell = Self {
            energy,
            age,
            genome_mode_id,
            genome_id,
            size_per_mass,
        };
        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size().extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(cell.get_genome_mode(genome_bank).colour))),
        )
    }

    #[must_use]
    pub fn get_genome_mode<'a>(&self, genome_bank: &'a GenomeBank) -> &'a GenomeMode {
        &genome_bank[self.genome_id][self.genome_mode_id]
    }

    #[must_use]
    pub fn get_mass(&self) -> f32 {
        #[allow(clippy::suboptimal_flops)]
        // Scale energy to get mass
        self.energy.powf(CELL_SIZE_SCALE_FACTOR)
    }

    #[must_use]
    pub fn get_size(&self) -> Vec2 {
        // Get masss then multiply that value to get the size
        Vec2::splat(self.get_mass() * self.size_per_mass)
    }

    pub fn split_into_daughter_bundles(
        &self,
        genome_bank: &GenomeBank,
        transform: &Transform,
        velocity: &Velocity,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<CellMaterial>>,
    ) -> Option<(CellBundle, CellBundle)> {
        let genome_mode = self.get_genome_mode(genome_bank);

        match genome_mode.split_type {
            CellSplitType::Age | CellSplitType::Energy => {
                // Check the correct parameter based on the parent's split type
                if (genome_mode.split_type == CellSplitType::Age && self.age >= genome_mode.split_age)
                    || (genome_mode.split_type == CellSplitType::Energy && self.energy >= genome_mode.split_energy)
                {
                    // Get the data for both daughters
                    let (d1, d2) = DaughterData::get_from_parent(self, velocity, transform, genome_bank);

                    let d1_bundle = d1.into_cell_bundle(genome_bank, meshes, materials);
                    let d2_bundle = d2.into_cell_bundle(genome_bank, meshes, materials);

                    // Return the bundles for the daughters
                    return Some((d1_bundle, d2_bundle));
                }
            }
            CellSplitType::Never => {}
        }

        None
    }
}

impl QuadTreeData for Cell {}
