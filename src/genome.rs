use std::ops::{Index, IndexMut};

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
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

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellSplitType {
    Energy,
    #[default]
    Age,
    Never,
}

#[derive(Component, Clone)]
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
            split_age: 2.0,
            split_energy: 20.,
            split_fraction: 0.5,
            split_angle: 0.,
            split_force: 10.,
            daughter_genomes: Default::default(),
        }
    }
}

pub const GENOME_MAX_NUM: usize = 9;

#[derive(Component, Default, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum GenomeId {
    #[default]
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,
}

impl From<GenomeId> for usize {
    fn from(value: GenomeId) -> Self {
        match value {
            GenomeId::M1 => 0,
            GenomeId::M2 => 1,
            GenomeId::M3 => 2,
            GenomeId::M4 => 3,
            GenomeId::M5 => 4,
            GenomeId::M6 => 5,
            GenomeId::M7 => 6,
            GenomeId::M8 => 7,
            GenomeId::M9 => 8,
        }
    }
}

impl From<usize> for GenomeId {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::M2,
            2 => Self::M3,
            3 => Self::M4,
            4 => Self::M5,
            5 => Self::M6,
            6 => Self::M7,
            7 => Self::M8,
            8 => Self::M9,
            _ => Self::M1,
        }
    }
}

impl std::fmt::Display for GenomeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M{}", Into::<usize>::into(*self) + 1)
    }
}

pub struct GenomeBank {
    bank: [Genome; GENOME_MAX_NUM],
}

impl Default for GenomeBank {
    fn default() -> Self {
        Self {
            bank: std::array::from_fn(|i| {
                let mut genome = Genome::new(i.into());
                genome.colour = Color::hsv((i as f32 / GENOME_MAX_NUM as f32) * 360.0, 0.8, 0.9); // Select a visually distinct colour for each genome

                genome
            }),
        }
    }
}

impl Index<GenomeId> for GenomeBank {
    type Output = Genome;

    fn index(&self, index: GenomeId) -> &Self::Output {
        &self.bank[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeId> for GenomeBank {
    fn index_mut(&mut self, index: GenomeId) -> &mut Self::Output {
        &mut self.bank[Into::<usize>::into(index)]
    }
}
