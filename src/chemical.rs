use bevy::prelude::*;

use crate::{helpers::random_vec2, state::GameState};

// Chemical parameters
const CHEMICAL_SIZE: f32 = 20.;
const CHEMICAL_ENERGY: f32 = 10.;
const CHEMICAL_SPAWN_RATE: f32 = 10.;
const CHEMICAL_MAX_NUM: usize = 400;

#[derive(Component)]
pub struct Chemical {
    pub energy: f32,
}

#[derive(Resource)]
pub struct ChemicalTimer(Timer);

impl Default for ChemicalTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1. / CHEMICAL_SPAWN_RATE, TimerMode::Repeating))
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn spawn_chemicals(
    mut commands: Commands,
    time: Res<Time>,
    state: Res<GameState>,
    mut timer: ResMut<ChemicalTimer>,
    chemicals: Query<(), With<Chemical>>,
) {
    timer.0.tick(time.delta());

    if chemicals.count() < CHEMICAL_MAX_NUM {
        // Spawn a random chemical depending on the spawn rate
        if timer.0.just_finished() {
            let chemical_bounds = (state.dish.size - CHEMICAL_SIZE) / 2.;

            let random_pos = random_vec2(chemical_bounds);

            commands.spawn((
                Chemical { energy: CHEMICAL_ENERGY },
                Sprite {
                    color: Color::linear_rgb(1., 0., 0.),
                    custom_size: Some(Vec2::splat(CHEMICAL_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(random_pos.x, random_pos.y, 0.),
            ));
        }
    }
}
