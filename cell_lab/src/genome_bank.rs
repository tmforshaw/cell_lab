use bevy::prelude::*;

use std::ops::{Index, IndexMut};

use crate::genome::{Genome, GenomeId};

pub const GENOME_MAX_NUM: usize = 9;
pub const GENOME_BANK_MAX_NUM: usize = 16;

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

pub enum GenomeBankId {}

#[derive(Resource)]
pub struct GenomeCollection {
    genome_collection: Vec<GenomeBank>,
}

impl Default for GenomeCollection {
    fn default() -> Self {
        Self {
            genome_collection: vec![GenomeBank::default()],
        }
    }
}

impl Index<(usize, GenomeId)> for GenomeCollection {
    type Output = Genome;

    fn index(&self, index: (usize, GenomeId)) -> &Self::Output {
        &self.genome_collection.get(index.0).unwrap_or_else(|| {
            panic!(
                "Index Error: Tried to access '{}' element of GenomeBank of length {}.",
                index.0,
                self.genome_collection.len()
            )
        })[index.1]
    }
}

impl IndexMut<(usize, GenomeId)> for GenomeCollection {
    fn index_mut(&mut self, index: (usize, GenomeId)) -> &mut Self::Output {
        &mut self
            .genome_collection
            .get_mut(index.0)
            .unwrap_or_else(|| panic!("Index Error: Tried to access element outside of GenomeBank range."))[index.1]
    }
}
