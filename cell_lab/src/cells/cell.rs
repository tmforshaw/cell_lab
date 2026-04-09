use bevy::prelude::*;

use crate::{
    cells::CellMaterial,
    game::{game_mode::GameMode, game_parameters::GameParameters},
    genomes::{CellSplitType, DaughterData, GenomeBank, GenomeId, GenomeMode, GenomeModeId},
    spatial_partitioning::quadtree::QuadTreeData,
};

#[derive(Component, Debug, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Bundle, Clone)]
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
}

impl Cell {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle(
        energy: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        param: &GameParameters,
        game_mode: &GameMode,
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
        };

        let genome_mode = cell.get_genome_mode(genome_bank);

        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size(param, game_mode).extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(
                genome_mode.colour,
                false,
                genome_mode.split_angle,
                genome_mode.split_fraction,
            ))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation(
        energy: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        param: &GameParameters,
        game_mode: &GameMode,
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
        };

        let genome_mode = cell.get_genome_mode(genome_bank);

        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size(param, game_mode).extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(
                genome_mode.colour,
                false,
                genome_mode.split_angle,
                genome_mode.split_fraction,
            ))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_age(
        energy: f32,
        age: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        param: &GameParameters,
        game_mode: &GameMode,
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
        };

        let genome_mode = cell.get_genome_mode(genome_bank);

        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.)).with_scale(cell.get_size(param, game_mode).extend(1.)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(
                genome_mode.colour,
                false,
                genome_mode.split_angle,
                genome_mode.split_fraction,
            ))),
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new_bundle_with_rotation_and_age(
        energy: f32,
        age: f32,
        genome_mode_id: GenomeModeId,
        genome_id: GenomeId,
        param: &GameParameters,
        game_mode: &GameMode,
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
        };

        let genome_mode = cell.get_genome_mode(genome_bank);

        CellBundle::new(
            cell.clone(),
            Velocity(velocity),
            Transform::from_translation(position.extend(1.))
                .with_scale(cell.get_size(param, game_mode).extend(1.))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(CellMaterial::new(
                genome_mode.colour,
                false,
                genome_mode.split_angle,
                genome_mode.split_fraction,
            ))),
        )
    }

    #[must_use]
    pub fn get_genome_mode<'a>(&self, genome_bank: &'a GenomeBank) -> &'a GenomeMode {
        &genome_bank[self.genome_id][self.genome_mode_id]
    }

    #[must_use]
    pub fn get_mass(&self, param: &GameParameters) -> f32 {
        #[allow(clippy::suboptimal_flops)]
        // Scale energy to get mass
        self.energy.powf(param.cell_parameters.mass_energy_scale_power)
    }

    #[must_use]
    pub fn get_size(&self, param: &GameParameters, game_mode: &GameMode) -> Vec2 {
        // Get masss then multiply that value to get the size
        Vec2::splat(self.get_mass(param) * param.get_cell_size_scale(game_mode))
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn split_into_daughter_bundles(
        &self,
        genome_bank: &GenomeBank,
        transform: &Transform,
        velocity: &Velocity,
        param: &GameParameters,
        game_mode: &GameMode,
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
                    let (d1, d2) = DaughterData::get_from_parent(self, velocity, transform, param, game_mode, genome_bank);

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
