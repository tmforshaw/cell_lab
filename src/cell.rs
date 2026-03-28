use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use rand::RngExt;

use std::f32::consts::PI;

use crate::{chemical::Chemical, state::GameState};

// Cell parameters
pub const CELL_ENERGY: f32 = 10.;
pub const CELL_MAX_VELOCITY: f32 = 100.;
const RANDOM_ACCELERATION: f32 = 10.;
pub const STARTING_CELL_NUM: u32 = 20;
const CELL_DIVISION_ENERGY: f32 = 20.;
const CELL_ENERGY_DECAY: f32 = 1.;

#[derive(Component, Clone)]
pub struct Cell {
    pub energy: f32,
    pub velocity: Vec2,
}

impl Cell {
    pub fn new_bundle(energy: f32, velocity: Vec2, position: Vec2, colour: Color) -> impl Bundle {
        let cell = Self { energy, velocity };
        (
            cell.clone(),
            Transform::from_translation(position.extend(0.)),
            Sprite {
                color: colour,
                custom_size: Some(cell.get_size()),
                ..default()
            },
        )
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::splat(self.energy * 2.)
    }
}

// Move cells smoothly
pub fn move_cells(time: Res<Time>, mut query: Query<(&mut Transform, &mut Cell)>) {
    let dt = time.delta().as_secs_f32();
    let mut rng = rand::rng();

    for (mut transform, mut cell) in query.iter_mut() {
        // Slight random acceleration
        cell.velocity += Vec2::new(
            rng.random_range(-RANDOM_ACCELERATION..RANDOM_ACCELERATION),
            rng.random_range(-RANDOM_ACCELERATION..RANDOM_ACCELERATION),
        ) * dt;

        // Clamp speed
        cell.velocity = cell
            .velocity
            .clamp(Vec2::splat(-CELL_MAX_VELOCITY), Vec2::splat(CELL_MAX_VELOCITY));

        // Move
        transform.translation += (cell.velocity * dt).extend(0.);
    }
}

pub fn bound_cells(state: Res<GameState>, mut query: Query<(&mut Transform, &mut Cell, &Sprite)>) {
    for (mut transform, mut cell, sprite) in query.iter_mut() {
        let size = if let Some(size) = sprite.custom_size {
            size
        } else {
            Vec2::splat(0.)
        };

        let bounds = (state.dish.size - size) / 2.;

        // X Bound Collision Resolution
        if transform.translation.x <= -bounds.x {
            cell.velocity.x *= -1.;
            transform.translation.x = -bounds.x;
        } else if transform.translation.x >= bounds.x {
            cell.velocity.x *= -1.;
            transform.translation.x = bounds.x;
        }

        // Y Bound Collision Resolution
        if transform.translation.y <= -bounds.y {
            cell.velocity.y *= -1.;
            transform.translation.y = -bounds.y;
        } else if transform.translation.y >= bounds.y {
            cell.velocity.y *= -1.;
            transform.translation.y = bounds.y;
        }
    }
}

pub fn cells_absorb_chemical(
    mut commands: Commands,
    mut cell_query: Query<(&Transform, &mut Cell, &mut Sprite), Without<Chemical>>,
    chemical_query: Query<(&Transform, &Chemical, &Sprite, Entity), Without<Cell>>,
) {
    for (cell_transform, mut cell, mut cell_sprite) in cell_query.iter_mut() {
        for (chemical_transform, chemical, chemical_sprite, chemical_entity) in chemical_query.iter() {
            // They both have sizes defined
            if let (Some(cell_size), Some(chemical_size)) = (cell_sprite.custom_size, chemical_sprite.custom_size) {
                // Generate bounding boxes
                let cell_aabb = Aabb2d::new(cell_transform.translation.xy(), cell_size / 2.);
                let chemical_aabb = Aabb2d::new(chemical_transform.translation.xy(), chemical_size / 2.);

                // Collision detected
                if cell_aabb.intersects(&chemical_aabb) {
                    // Gain energy then resize cell based on new energy
                    cell.energy += chemical.energy;
                    cell_sprite.custom_size = Some(cell.get_size());

                    // Despawn the chemical
                    commands.entity(chemical_entity).despawn();
                }
            }
        }
    }
}

pub fn cells_do_meiosis(mut commands: Commands, mut query: Query<(&Transform, &mut Cell, &mut Sprite)>) {
    for (transform, mut cell, mut sprite) in query.iter_mut() {
        if cell.energy > CELL_DIVISION_ENERGY {
            // Generate a random angle for the velocity
            let angle = rand::rng().random::<f32>() * PI;

            // Rotate the velocity to match these angles
            let v1 = cell.velocity.rotate(Vec2::from_angle(angle / 2.));
            let v2 = cell.velocity.rotate(Vec2::from_angle(-angle / 2.));

            // Scale the magnitude so it conserves momentum
            let magnitude_scale = cell.velocity.length() / (v1 + v2).length();

            // Create a new cell
            commands.spawn(Cell::new_bundle(
                cell.energy / 2.,
                v2 * magnitude_scale,
                transform.translation.xy(),
                sprite.color,
            ));

            // Change cell energy and velocity, then resize cell
            cell.energy /= 2.;
            cell.velocity = v1 * magnitude_scale;
            sprite.custom_size = Some(cell.get_size());
        }
    }
}

pub fn cell_decay(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Cell, &mut Sprite, Entity)>) {
    let dt = time.delta().as_secs_f32();

    for (mut cell, mut sprite, entity) in query.iter_mut() {
        // Reduce energy
        cell.energy -= CELL_ENERGY_DECAY * dt;

        // Remove cell if its too small
        if cell.energy <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Resize the cell
            sprite.custom_size = Some(cell.get_size());
        }
    }
}
