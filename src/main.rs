use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use rand::RngExt;

use std::f32::consts::PI;

// Cell parameters
const CELL_ENERGY: f32 = 10.;
const CELL_MAX_VELOCITY: f32 = 100.;
const RANDOM_ACCELERATION: f32 = 10.;
const STARTING_CELL_NUM: u32 = 20;
const CELL_DIVISION_ENERGY: f32 = 20.;
const CELL_ENERGY_DECAY: f32 = 1.;

// Dish parameters
const DISH_SIZE: Vec2 = Vec2::new(1200., 1200.);
const DISH_COLOUR: Color = Color::linear_rgb(0.2, 0.2, 0.2);

// Chemical parameters
const CHEMICAL_SIZE: f32 = 20.;
const CHEMICAL_ENERGY: f32 = 10.;
const CHEMICAL_SPAWN_RATE: f32 = 10.;
const CHEMICAL_MAX_NUM: usize = 400;

// Components
#[derive(Component, Clone)]
struct Cell {
    energy: f32,
    velocity: Vec2,
}

impl Cell {
    fn new_bundle(energy: f32, velocity: Vec2, position: Vec2, colour: Color) -> impl Bundle {
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

    fn get_size(&self) -> Vec2 {
        Vec2::splat(self.energy * 2.)
    }
}

#[derive(Component)]
struct Chemical {
    energy: f32,
}

#[derive(Resource)]
struct ChemicalTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_chemicals,
                move_cells,
                bound_cells,
                cells_absorb_chemical,
                cells_do_meiosis,
            ),
        )
        .add_systems(PostUpdate, cell_decay)
        .run();
}

// Spawn cells and chemicals
fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);

    // Show dish

    commands.spawn(Sprite {
        color: DISH_COLOUR,
        custom_size: Some(DISH_SIZE),
        ..default()
    });

    let mut rng = rand::rng();

    // Spawn cells
    for _ in 0..STARTING_CELL_NUM {
        commands.spawn(Cell::new_bundle(
            CELL_ENERGY,
            Vec2::new(
                rng.random_range(-CELL_MAX_VELOCITY..CELL_MAX_VELOCITY),
                rng.random_range(-CELL_MAX_VELOCITY..CELL_MAX_VELOCITY),
            ),
            Vec2::new(
                rng.random_range((-DISH_SIZE.x / 2.)..(DISH_SIZE.x / 2.)),
                rng.random_range((-DISH_SIZE.y / 2.)..(DISH_SIZE.y / 2.)),
            ),
            Color::linear_rgb(0., 1., 0.),
        ));
    }

    commands.insert_resource(ChemicalTimer(Timer::from_seconds(
        1. / CHEMICAL_SPAWN_RATE,
        TimerMode::Repeating,
    )))
}

// Move cells smoothly
fn move_cells(time: Res<Time>, mut query: Query<(&mut Transform, &mut Cell)>) {
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

fn bound_cells(mut query: Query<(&mut Transform, &mut Cell, &Sprite)>) {
    for (mut transform, mut cell, sprite) in query.iter_mut() {
        let size = if let Some(size) = sprite.custom_size {
            size
        } else {
            Vec2::splat(0.)
        };

        let bounds = (DISH_SIZE - size) / 2.;

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

fn cells_absorb_chemical(
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

fn cells_do_meiosis(mut commands: Commands, mut query: Query<(&Transform, &mut Cell, &mut Sprite)>) {
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

fn spawn_chemicals(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ChemicalTimer>,
    chemicals: Query<(), With<Chemical>>,
) {
    timer.0.tick(time.delta());

    if chemicals.count() < CHEMICAL_MAX_NUM {
        // Spawn a random chemical depending on the spawn rate
        if timer.0.just_finished() {
            let mut rng = rand::rng();

            let chemical_bounds = (DISH_SIZE - CHEMICAL_SIZE) / 2.;
            commands.spawn((
                Chemical { energy: CHEMICAL_ENERGY },
                Sprite {
                    color: Color::linear_rgb(1., 0., 0.),
                    custom_size: Some(Vec2::splat(CHEMICAL_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    rng.random_range(-chemical_bounds.x..chemical_bounds.x),
                    rng.random_range(-chemical_bounds.y..chemical_bounds.y),
                    0.,
                ),
            ));
        }
    }
}

fn cell_decay(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Cell, &mut Sprite, Entity)>) {
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
