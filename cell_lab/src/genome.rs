use std::f32::consts::PI;

use bevy::prelude::*;
use cell_lab_macros::generate_enum;

use crate::{cell::Cell, genome_bank::GenomeBank};

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CellType {
    #[default]
    Phagocyte,
    Photocyte,
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Phagocyte => "Phagocyte",
                Self::Photocyte => "Photocyte",
            },
        )
    }
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellSplitType {
    Energy,
    #[default]
    Age,
    Never,
}

#[derive(Component, Debug, Clone)]
pub struct Genome {
    pub id: GenomeId,
    pub cell_type: CellType,
    pub colour: Color,
    pub split_type: CellSplitType,
    pub split_age: f32,
    pub split_energy: f32,
    pub split_fraction: f32,
    pub split_angle: f32,
    pub split_force: f32,
    pub daughter_genomes: (GenomeId, GenomeId),
    // pub daughter_angles: (f32, f32),
}

impl Genome {
    #[must_use]
    pub fn new(id: GenomeId) -> Self {
        Self {
            id,
            daughter_genomes: (id, id),
            ..default()
        }
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            id: GenomeId::default(),
            cell_type: CellType::default(),
            colour: Color::default(),
            split_type: CellSplitType::default(),
            split_age: 10.0,
            split_energy: 20.,
            split_fraction: 0.5,
            split_angle: 0.,
            split_force: 15.,
            daughter_genomes: Default::default(),
            // daughter_angles: (0., 0.),
        }
    }
}

generate_enum!(GenomeId, M, GENOME_MAX_NUM, 9);

pub struct DaughterData {
    pub energy: f32,
    pub genome_id: GenomeId,
    pub colour: Color, // TODO Remove this in favour of finding it from the genome
    pub velocity: Vec2,
    pub position: Vec2,
}

#[must_use]
pub fn get_daughter_data(
    parent_cell: &Cell,
    parent_genome_id: GenomeId,
    parent_position: Vec2,
    parent_scale: Vec2,
    genome_bank: &GenomeBank,
) -> (DaughterData, DaughterData) {
    // Split energy depending on split fraction
    let d1_energy = parent_cell.energy * genome_bank[parent_genome_id].split_fraction;
    let d2_energy = parent_cell.energy - d1_energy;

    // Set genome_id according to genome bank
    let d1_genome_id = genome_bank[parent_genome_id].daughter_genomes.0;
    let d2_genome_id = genome_bank[parent_genome_id].daughter_genomes.1;

    // Set colour according to genome bank
    let d1_colour = genome_bank[d1_genome_id].colour;
    let d2_colour = genome_bank[d2_genome_id].colour;

    // Give velocity depending on split angle
    let velocity_mag = genome_bank[parent_genome_id].split_force * 0.1;
    let d1_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(genome_bank[parent_genome_id].split_angle - PI / 2.));
    let d2_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(genome_bank[parent_genome_id].split_angle + PI / 2.));

    // Offset the daughters
    let d1_position = parent_position + (parent_scale * genome_bank[parent_genome_id].split_fraction) / 2. * d1_velocity;
    let d2_position = parent_position + (parent_scale * (1. - genome_bank[parent_genome_id].split_fraction)) / 2. * d2_velocity;

    (
        // Set the first daughter's parameters
        DaughterData {
            energy: d1_energy,
            genome_id: d1_genome_id,
            colour: d1_colour,
            velocity: d1_velocity,
            position: d1_position,
        },
        // Set the second daughter's parameters
        DaughterData {
            energy: d2_energy,
            genome_id: d2_genome_id,
            colour: d2_colour,
            velocity: d2_velocity,
            position: d2_position,
        },
    )
}
