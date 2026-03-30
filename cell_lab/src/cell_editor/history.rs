use bevy::prelude::*;

use crate::{cell::Cell, cell_editor::systems::CellTimeOfBirth, genome::GenomeId};

// TODO
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SplitHistoryData {
    pub simulation_age: f32,
    pub parent: Cell,
    pub parent_genome_id: GenomeId,
    pub parent_position: Vec2,
    pub parent_velocity: Vec2,
    pub parent_time_of_birth: CellTimeOfBirth,
}

#[derive(Default, Debug)]
pub struct SplitHistory {
    current_idx: Option<usize>,
    history: Vec<SplitHistoryData>,
}

impl SplitHistory {
    pub const fn increment_current(&mut self) {
        self.current_idx = Some(match self.current_idx {
            Some(idx) if idx == self.history.len() - 1 => idx,
            Some(idx) => idx + 1,
            None => 0,
        });
    }

    pub const fn decrement_current(&mut self) {
        self.current_idx = match self.current_idx {
            Some(0) | None => None,
            Some(idx) => Some(idx - 1),
        };
    }

    fn clear_after_current(&mut self) {
        match self.current_idx {
            Some(idx) => {
                let _ = self.history.split_off(idx + 1);
            }
            None => {
                self.history = Vec::new();
            }
        }
    }

    pub fn insert(&mut self, data: SplitHistoryData) {
        self.clear_after_current();
        self.history.push(data);
        self.increment_current();
    }

    #[must_use]
    pub fn get(&self) -> Option<SplitHistoryData> {
        self.current_idx.map(|idx| self.history[idx].clone())
    }
}
