use bevy::prelude::*;

use crate::cell_editor::CellEditorState;
use crate::chemical::Chemical;
use crate::dish::{Dish, DishMarker};

use crate::{
    cell::{CELL_ENERGY, CELL_MAX_VELOCITY, Cell, STARTING_CELL_NUM},
    helpers::random_vec2,
};

#[derive(Resource, Default)]
pub struct GameState {
    pub dish: Dish,
}

impl GameState {
    #[must_use]
    pub const fn new(dish: Dish) -> Self {
        Self { dish }
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum GameMode {
    #[default]
    Play,
    CellEditor,
}

// ---------------------------- Play Mode -----------------------------
#[allow(clippy::needless_pass_by_value)]
pub fn init_play_mode(mut commands: Commands, state: Res<GameState>) {
    // Show dish
    commands.spawn(state.dish.into_bundle());

    // Spawn cells
    for _ in 0..STARTING_CELL_NUM {
        commands.spawn(Cell::new_bundle(
            CELL_ENERGY,
            random_vec2(Vec2::splat(CELL_MAX_VELOCITY)),
            random_vec2(state.dish.size / 2.),
            Color::linear_rgb(0., 1., 0.),
        ));
    }
}

pub fn exit_play_mode(
    mut commands: Commands,
    dishes: Query<Entity, With<DishMarker>>,
    cells: Query<Entity, With<Cell>>,
    chemicals: Query<Entity, With<Chemical>>,
) {
    // Remove the dish
    for entity in dishes.iter() {
        commands.entity(entity).despawn();
    }

    // Remove the cells
    for entity in cells.iter() {
        commands.entity(entity).despawn();
    }

    // Remove the chemicals
    for entity in chemicals.iter() {
        commands.entity(entity).despawn();
    }
}

// ------------------------- Cell Editor Mode --------------------------

pub fn init_cell_editor_mode(mut commands: Commands) {
    commands.init_resource::<CellEditorState>();

    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::splat(1200.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        DishMarker,
    ));
}

pub fn exit_cell_editor_mode(mut commands: Commands, dishes: Query<Entity, With<DishMarker>>) {
    commands.remove_resource::<CellEditorState>();

    for entity in dishes.iter() {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------
