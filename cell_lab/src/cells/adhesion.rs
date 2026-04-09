use bevy::prelude::*;

use crate::{
    cells::{Cell, Velocity},
    game::{game_mode::GameMode, game_parameters::GameParameters},
};

#[derive(Component)]
pub struct Adhesion {
    pub other: Entity,
    pub params: AdhesionParameters,
}

#[derive(Debug, Clone)]
pub struct AdhesionParameters {
    pub rest_length: f32,
    pub strength: f32,
    pub damping: f32,
}

impl Default for AdhesionParameters {
    fn default() -> Self {
        Self {
            rest_length: 50.,
            strength: 500.0,
            damping: 1.0,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::trivially_copy_pass_by_ref)]
fn apply_adhesion_force(
    pos_1: &mut Vec3,
    velocity_1: &mut Velocity,
    cell_1: &Cell,
    pos_2: &mut Vec3,
    velocity_2: &mut Velocity,
    cell_2: &Cell,
    adhesion: &Adhesion,
    delta_time: f32,
    param: &GameParameters,
    game_mode: &GameMode,
) {
    let mass_1 = cell_1.get_mass(param);
    let mass_2 = cell_2.get_mass(param);

    let centre_of_mass = (mass_1 * pos_1.xy() + mass_2 * pos_2.xy()) / (mass_1 + mass_2);

    let relative_pos_1 = pos_1.xy() - centre_of_mass;
    let relative_pos_2 = pos_2.xy() - centre_of_mass;

    let centre_of_mass_velocity = (mass_1 * velocity_1.0 + mass_2 * velocity_2.0) / (mass_1 + mass_2);

    // Cluster angular velocity
    let angular_momentum = relative_pos_1.perp_dot(mass_1 * velocity_1.0) + relative_pos_2.perp_dot(mass_2 * velocity_2.0);
    #[allow(clippy::suboptimal_flops)]
    let total_moment_of_intertia = mass_1 * relative_pos_1.length_squared() + mass_2 * relative_pos_2.length_squared();
    let angular_velocity = angular_momentum / total_moment_of_intertia; // In radians per second

    // Update positions based on angular velocity
    *pos_1 = (centre_of_mass + relative_pos_1.rotate(Vec2::from_angle(angular_velocity * delta_time))).extend(pos_1.z);
    *pos_2 = (centre_of_mass + relative_pos_2.rotate(Vec2::from_angle(angular_velocity * delta_time))).extend(pos_2.z);

    // Update velocities based on angular velocity
    velocity_1.0 = centre_of_mass_velocity + angular_velocity * relative_pos_1.perp();
    velocity_2.0 = centre_of_mass_velocity + angular_velocity * relative_pos_2.perp();

    let adhesion_distance = adhesion
        .params
        .rest_length
        .min((cell_1.get_size(param, game_mode).x + cell_2.get_size(param, game_mode).x) * 0.33);

    // Force the distance between them to be the same
    let delta = (pos_2.xy() - pos_1.xy()) - adhesion_distance;
    let correction = delta; // Add spring constant here
    *pos_1 += (correction * 0.5).extend(0.);
    *pos_2 -= (correction * 0.5).extend(0.);
}

#[allow(clippy::needless_pass_by_value)]
pub fn apply_adhesion_system(
    adhesions: Query<(Entity, &Adhesion)>,
    mut cells: Query<(&Cell, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
    param: Res<GameParameters>,
    game_mode: Res<State<GameMode>>,
) {
    // Iterate over all adhesions
    for (entity, adhesion) in &adhesions {
        let other = adhesion.other;

        // Enforce the ordering of the pairs
        if entity >= other {
            continue;
        }

        if let Ok(
            [
                (cell_1, mut transform_1, mut velocity_1),
                (cell_2, mut transform_2, mut velocity_2),
            ],
        ) = cells.get_many_mut([entity, other])
        {
            apply_adhesion_force(
                &mut transform_1.translation,
                &mut velocity_1,
                cell_1,
                &mut transform_2.translation,
                &mut velocity_2,
                cell_2,
                adhesion,
                time.delta_secs(),
                &param,
                &game_mode,
            );
        }
    }
}
