use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use std::ops::{Index, IndexMut};

use cell_lab_macros::generate_enum;

use crate::genomes::{GENOME_MODE_MAX_NUM, GenomeMode, GenomeModeId, genome_mode::colour_from_genome_mode_id};

#[derive(Serialize, Deserialize)]
pub struct Genome {
    pub initial: GenomeModeId,
    modes: [GenomeMode; GENOME_MODE_MAX_NUM],
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            initial: GenomeModeId::default(),
            modes: std::array::from_fn(|i| {
                let mut genome_mode = GenomeMode::new(i.into());

                // Select a visually distinct colour for each genome mode
                genome_mode.colour = colour_from_genome_mode_id(i.into());
                genome_mode
            }),
        }
    }
}

impl Index<GenomeModeId> for Genome {
    type Output = GenomeMode;

    fn index(&self, index: GenomeModeId) -> &Self::Output {
        &self.modes[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeModeId> for Genome {
    fn index_mut(&mut self, index: GenomeModeId) -> &mut Self::Output {
        &mut self.modes[Into::<usize>::into(index)]
    }
}

generate_enum!(GenomeId, B, GENOME_MAX_NUM, 16);

#[derive(Resource)]
pub struct GenomeCollection {
    genome_collection: [Genome; GENOME_MAX_NUM],
}

impl Default for GenomeCollection {
    fn default() -> Self {
        Self {
            genome_collection: std::array::from_fn(|_| Genome::default()),
        }
    }
}

impl Index<GenomeId> for GenomeCollection {
    type Output = Genome;

    fn index(&self, index: GenomeId) -> &Self::Output {
        &self.genome_collection[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeId> for GenomeCollection {
    fn index_mut(&mut self, index: GenomeId) -> &mut Self::Output {
        &mut self.genome_collection[Into::<usize>::into(index)]
    }
}
