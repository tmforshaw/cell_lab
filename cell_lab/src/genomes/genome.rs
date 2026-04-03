use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use std::ops::{Index, IndexMut};

use cell_lab_macros::generate_enum;

use crate::{
    game::game_parameters::GameParameters,
    genomes::{GENOME_MODE_MAX_NUM, GenomeMode, GenomeModeId, genome_mode::colour_from_genome_mode_id},
};

#[derive(Serialize, Deserialize)]
pub struct Genome {
    pub initial: GenomeModeId,
    modes: [GenomeMode; GENOME_MODE_MAX_NUM],
}

impl Genome {
    #[must_use]
    pub fn new_from_parameters(param: &GameParameters) -> Self {
        Self {
            initial: GenomeModeId::default(),
            modes: std::array::from_fn(|i| {
                let mut genome_mode = GenomeMode::new(i.into());

                // Select a visually distinct colour for each genome mode
                genome_mode.colour = colour_from_genome_mode_id(i.into(), param);
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
pub struct GenomeBank {
    genomes: [Genome; GENOME_MAX_NUM],
}

impl GenomeBank {
    #[must_use]
    pub fn new_from_parameters(param: &GameParameters) -> Self {
        Self {
            genomes: std::array::from_fn(|_| Genome::new_from_parameters(param)),
        }
    }
}

impl Index<GenomeId> for GenomeBank {
    type Output = Genome;

    fn index(&self, index: GenomeId) -> &Self::Output {
        &self.genomes[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeId> for GenomeBank {
    fn index_mut(&mut self, index: GenomeId) -> &mut Self::Output {
        &mut self.genomes[Into::<usize>::into(index)]
    }
}
