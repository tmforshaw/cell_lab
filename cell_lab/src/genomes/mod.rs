pub mod daughters;
pub mod genome;
pub mod genome_bank;

// pub use daughters::{DaughterData, get_daughter_data};
pub use daughters::DaughterData;
pub use genome::{CellSplitType, CellType, GENOME_MAX_NUM, Genome, GenomeId};
pub use genome_bank::{GENOME_BANK_MAX_NUM, GenomeBank, GenomeBankId, GenomeCollection};
