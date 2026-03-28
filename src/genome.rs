use bevy::prelude::*;

#[derive(Component)]
pub enum CellType {
    Phagocyte,
}

#[derive(Component)]
pub struct Genome {
    pub id: GenomeId,
    pub cell_type: CellType,
    pub colour: Color,
    pub split_fraction: f32,
    pub split_threshold: f32,
    pub offspring_genomes: (GenomeId, GenomeId),
}

#[derive(Component)]
pub enum GenomeId {
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
