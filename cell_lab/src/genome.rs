use std::f32::consts::PI;

use bevy::prelude::*;
use cell_lab_macros::generate_enum;

use crate::{
    cell::{CELL_SPLIT_PADDING, Cell, Velocity},
    genome_bank::GenomeCollection,
};

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
    #[default]
    Energy,
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
    pub daughter_angles: (f32, f32),
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
            daughter_angles: (0., 0.),
        }
    }
}

generate_enum!(GenomeId, M, GENOME_MAX_NUM, 9);

pub struct DaughterData {
    pub energy: f32,
    pub genome_id: GenomeId,
    pub velocity: Vec2,
    pub position: Vec2,
    pub rotation: f32,
}

#[must_use]
pub fn get_daughter_data(
    parent: &Cell,
    velocity: &Velocity,
    transform: &Transform,
    genome_collection: &GenomeCollection,
) -> (DaughterData, DaughterData) {
    let parent_genome = &genome_collection[parent.genome_bank_id][parent.genome_id];

    // Split energy depending on split fraction
    let d1_energy = parent.energy * parent_genome.split_fraction;
    let d2_energy = parent.energy - d1_energy;

    // Set genome_id according to genome bank
    let d1_genome_id = parent_genome.daughter_genomes.0;
    let d2_genome_id = parent_genome.daughter_genomes.1;

    // Also ensure to rotate the split direction based on the parent's rotation
    let parent_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

    // Give new velocity depending on split angle
    let velocity_mag = parent_genome.split_force;
    let d1_new_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(parent_genome.split_angle - PI / 2. + parent_angle));
    let d2_new_velocity = velocity_mag * Vec2::Y.rotate(Vec2::from_angle(parent_genome.split_angle + PI / 2. + parent_angle));

    // Calculate the amount of velocity to give to each daughter based on split fraction
    let d1_velocity_from_parent = velocity.0 * parent_genome.split_fraction.sqrt();
    let d2_velocity_from_parent = velocity.0 * (1. - parent_genome.split_fraction).sqrt();

    // Add up the daughter's new velocity and the velocity from the parent
    let d1_velocity = d1_new_velocity + d1_velocity_from_parent;
    let d2_velocity = d2_new_velocity + d2_velocity_from_parent;

    // Offset the daughters by their width (plus a little bit of padding)
    let d1_position = transform.translation.xy()
        + (transform.scale.xy() * parent_genome.split_fraction) / 2. * d1_new_velocity.normalize() * CELL_SPLIT_PADDING;
    let d2_position = transform.translation.xy()
        + (transform.scale.xy() * (1. - parent_genome.split_fraction)) / 2. * d2_new_velocity.normalize() * CELL_SPLIT_PADDING;

    (
        // Set the first daughter's parameters
        DaughterData {
            energy: d1_energy,
            genome_id: d1_genome_id,
            velocity: d1_velocity,
            position: d1_position,
            rotation: parent_genome.daughter_angles.0,
        },
        // Set the second daughter's parameters
        DaughterData {
            energy: d2_energy,
            genome_id: d2_genome_id,
            velocity: d2_velocity,
            position: d2_position,
            rotation: parent_genome.daughter_angles.1,
        },
    )
}
