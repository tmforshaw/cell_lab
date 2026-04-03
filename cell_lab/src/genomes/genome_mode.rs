use bevy::prelude::*;
use cell_lab_macros::generate_enum;
use serde::{Deserialize, Serialize};

const GENOME_MODE_INDEX_COLOUR_OFFSET: f32 = 120.;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
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

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CellSplitType {
    #[default]
    Energy,
    Age,
    Never,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GenomeMode {
    pub id: GenomeModeId,
    pub cell_type: CellType,
    pub colour: Color,
    pub split_type: CellSplitType,
    pub split_age: f32,
    pub split_energy: f32,
    pub split_fraction: f32,
    pub split_angle: f32,
    pub split_force: f32,
    pub daughter_genome_modes: (GenomeModeId, GenomeModeId),
    pub daughter_angles: (f32, f32),
}

impl GenomeMode {
    #[must_use]
    pub fn new(id: GenomeModeId) -> Self {
        Self {
            id,
            daughter_genome_modes: (id, id),
            ..default()
        }
    }
}

impl Default for GenomeMode {
    fn default() -> Self {
        Self {
            id: GenomeModeId::default(),
            cell_type: CellType::default(),
            colour: Color::default(),
            split_type: CellSplitType::default(),
            split_age: 5.0,
            split_energy: 20.,
            split_fraction: 0.5,
            split_angle: 0.,
            split_force: 15.,
            daughter_genome_modes: Default::default(),
            daughter_angles: (0., 0.),
        }
    }
}

generate_enum!(GenomeModeId, M, GENOME_MODE_MAX_NUM, 9);

#[must_use]
pub fn colour_from_genome_mode_id(genome_mode_id: GenomeModeId) -> Color {
    Color::hsv(
        (Into::<usize>::into(genome_mode_id) as f32 / GENOME_MODE_MAX_NUM as f32)
            .mul_add(360.0, GENOME_MODE_INDEX_COLOUR_OFFSET)
            .rem_euclid(360.),
        0.8,
        0.9,
    )
}
