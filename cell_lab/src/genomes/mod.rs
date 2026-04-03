pub mod daughters;
pub mod genome;
pub mod genome_mode;

pub use daughters::DaughterData;
pub use genome::{GENOME_MAX_NUM, Genome, GenomeCollection, GenomeId};
pub use genome_mode::{CellSplitType, CellType, GENOME_MODE_MAX_NUM, GenomeMode, GenomeModeId};
