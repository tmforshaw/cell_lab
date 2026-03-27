use bevy::prelude::*;

use rand::RngExt;

// Cell parameters
const CELL_ENERGY: f32 = 10.;
const CELL_MAX_SIZE: f32 = 20.;
const CELL_MAX_VELOCITY: f32 = 50.;
const RANDOM_ACCELERATION: f32 = 10.;
const STARTING_CELL_NUM: u32 = 20;

// Dish parameters
const DISH_SIZE: Vec2 = Vec2::new(800., 800.);
const DISH_COLOUR: Color = Color::linear_rgb(0.2, 0.2, 0.2);

// Chemical parameters
const CHEMICAL_SIZE: f32 = 10.;
const STARTING_CHEMICAL_NUM: u32 = 50;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_cells, bound_cells))
        .run();
}

// Components
#[derive(Component)]
struct Cell {
    energy: f32,
    velocity: Vec2,
}

impl Cell {
    fn new_bundle(energy: f32, velocity: Vec2, position: Vec2, colour: Color, radius: f32) -> impl Bundle {
        (
            Self { energy, velocity },
            Transform::from_translation(position.extend(0.)),
            Sprite {
                color: colour,
                custom_size: Some(Vec2::splat(radius)),
                ..default()
            },
        )
    }
}

#[derive(Component)]
struct Chemical {
    energy: f32,
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
            CELL_MAX_SIZE,
        ));
    }

    // Spawn chemicals
    let chemical_bounds = (DISH_SIZE - CHEMICAL_SIZE) / 2.;
    for _ in 0..STARTING_CHEMICAL_NUM {
        commands.spawn((
            Chemical { energy: 10.0 },
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
