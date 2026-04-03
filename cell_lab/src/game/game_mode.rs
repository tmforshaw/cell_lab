use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum GameMode {
    #[default]
    Simulation,
    CellEditor,
}
