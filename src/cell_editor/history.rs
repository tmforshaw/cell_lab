use bevy::prelude::*;

use crate::genome::GenomeId;

#[derive(Resource)]
pub struct SplitHistory {
    history: Vec<SplitHistoryData>,
}

pub struct SplitHistoryData {
    simulation_age: f32,
    parent_genome_id: GenomeId,
}
