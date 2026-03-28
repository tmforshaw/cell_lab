use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d};

use crate::{helpers::random_vec2, state::GameState};

// Chemical parameters
const CHEMICAL_SIZE: f32 = 20.;
const CHEMICAL_ENERGY: f32 = 10.;
const CHEMICAL_SPAWN_RATE: f32 = 10.;
const CHEMICAL_MAX_NUM: usize = 400;
const CHEMICAL_COLOUR: Color = Color::linear_rgba(0.5, 0.1, 0.1, 0.75);

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
    mut materials: ResMut<Assets<ChemicalMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
                Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
                MeshMaterial2d(materials.add(ChemicalMaterial::new(CHEMICAL_COLOUR))),
                Transform::from_xyz(random_pos.x, random_pos.y, 0.5).with_scale(Vec2::splat(CHEMICAL_SIZE).extend(1.)),
            ));
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ChemicalMaterial {
    #[uniform(0)]
    pub colour: Vec4,
}

impl Material2d for ChemicalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chemical_material.wgsl".into()
    }
}

impl ChemicalMaterial {
    #[must_use]
    pub fn new(colour: Color) -> Self {
        Self {
            colour: colour.to_linear().to_vec4(),
        }
    }
}
