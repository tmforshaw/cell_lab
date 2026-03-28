use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CellType {
    #[default]
    Phagocyte,
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Phagocyte => "Phagocyte",
            },
        )
    }
}

#[derive(Component, Clone)]
pub struct Genome {
    pub id: GenomeId,
    pub cell_type: CellType,
    pub colour: Color,
    pub split_fraction: f32,
    pub split_threshold: f32,
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
            split_fraction: 0.5,
            split_threshold: 0.5,
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
            GenomeId::M1 => 1,
            GenomeId::M2 => 2,
            GenomeId::M3 => 3,
            GenomeId::M4 => 4,
            GenomeId::M5 => 5,
            GenomeId::M6 => 6,
            GenomeId::M7 => 7,
            GenomeId::M8 => 8,
            GenomeId::M9 => 9,
        }
    }
}

impl std::fmt::Display for GenomeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M{}", Into::<usize>::into(*self))
    }
}
