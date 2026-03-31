use bevy::prelude::*;

use std::ops::{Index, IndexMut};

use cell_lab_macros::generate_enum;

use crate::genome::{GENOME_MAX_NUM, Genome, GenomeId};

pub struct GenomeBank {
    pub initial: GenomeId,
    bank: [Genome; GENOME_MAX_NUM],
}

impl Default for GenomeBank {
    fn default() -> Self {
        Self {
            initial: GenomeId::default(),
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
