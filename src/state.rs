use bevy::prelude::*;

use crate::dish::Dish;

#[derive(Resource, Default)]
pub struct GameState {
    pub dish: Dish,
    pub mode: GameMode,
}

impl GameState {
    pub fn new(dish: Dish, mode: GameMode) -> Self {
        Self { dish, mode }
    }
}

#[derive(Resource, Default, PartialEq, PartialOrd, Ord, Eq)]
pub enum GameMode {
    #[default]
    Play,
    CellEditor,
}

pub fn play_mode_criteria(state: Res<GameState>) -> bool {
    state.mode == GameMode::Play
}

pub fn cell_editor_mode_criteria(state: Res<GameState>) -> bool {
    state.mode == GameMode::CellEditor
}
