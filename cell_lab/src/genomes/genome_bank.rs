use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use std::ops::{Index, IndexMut};

use cell_lab_macros::generate_enum;

use crate::genomes::{GENOME_MODE_MAX_NUM, GenomeMode, GenomeModeId, genome_mode::colour_from_genome_mode_id};

#[derive(Serialize, Deserialize)]
pub struct GenomeBank {
    pub initial: GenomeModeId,
    bank: [GenomeMode; GENOME_MODE_MAX_NUM],
}

impl Default for GenomeBank {
    fn default() -> Self {
        Self {
            initial: GenomeModeId::default(),
            bank: std::array::from_fn(|i| {
                let mut genome_mode = GenomeMode::new(i.into());

                // Select a visually distinct colour for each genome mode
                genome_mode.colour = colour_from_genome_mode_id(i.into());
                genome_mode
            }),
        }
    }
}

impl Index<GenomeModeId> for GenomeBank {
    type Output = GenomeMode;

    fn index(&self, index: GenomeModeId) -> &Self::Output {
        &self.bank[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeModeId> for GenomeBank {
    fn index_mut(&mut self, index: GenomeModeId) -> &mut Self::Output {
        &mut self.bank[Into::<usize>::into(index)]
    }
}

generate_enum!(GenomeBankId, B, GENOME_BANK_MAX_NUM, 16);

#[derive(Resource)]
pub struct GenomeCollection {
    genome_collection: [GenomeBank; GENOME_BANK_MAX_NUM],
}

impl Default for GenomeCollection {
    fn default() -> Self {
        Self {
            genome_collection: std::array::from_fn(|_| GenomeBank::default()),
        }
    }
}

impl Index<GenomeBankId> for GenomeCollection {
    type Output = GenomeBank;

    fn index(&self, index: GenomeBankId) -> &Self::Output {
        &self.genome_collection[Into::<usize>::into(index)]
    }
}

impl IndexMut<GenomeBankId> for GenomeCollection {
    fn index_mut(&mut self, index: GenomeBankId) -> &mut Self::Output {
        &mut self.genome_collection[Into::<usize>::into(index)]
    }
}
