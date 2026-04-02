use bevy::prelude::*;

use crate::cell_editor::logical_cell::LogicalCell;

#[derive(Clone)]
pub struct CellsSnapshot {
    pub cells: Vec<LogicalCell>,
    pub time: f32,
}

#[derive(Resource, Default)]
pub struct CellEditorSimulationState {
    pub cells: Vec<LogicalCell>,
    pub current_time: f32,
}

#[derive(Resource, Default)]
pub struct CellHistoryCache {
    snapshots: Vec<CellsSnapshot>,
}

impl CellHistoryCache {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn get_closest_past_snapshot(&self, target_time: f32) -> Option<&CellsSnapshot> {
        #[allow(clippy::unwrap_used)]
        self.snapshots
            .iter()
            .filter(|snap| snap.time <= target_time && snap.time.is_finite())
            .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
    }

    #[must_use]
    pub fn should_store_snapshot(&self, time: f32) -> bool {
        const SNAPSHOT_INTERVAL: f32 = 0.5;

        self.snapshots.last().is_none_or(|last| time - last.time >= SNAPSHOT_INTERVAL)
    }

    pub fn trim(&mut self) {
        const MAX_SNAPSHOTS: usize = 64;

        if self.snapshots.len() > MAX_SNAPSHOTS {
            let excess = self.snapshots.len() - MAX_SNAPSHOTS;
            self.snapshots.drain(0..excess);
        }
    }

    pub fn insert(&mut self, cells: &[LogicalCell], time: f32) {
        self.snapshots.push(CellsSnapshot {
            cells: cells.into(),
            time,
        });
    }

    pub fn clear(&mut self) {
        self.snapshots.clear();
    }
}
