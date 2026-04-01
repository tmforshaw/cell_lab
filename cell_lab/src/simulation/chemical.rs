use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d};

use crate::spatial_partitioning::quadtree::QuadTreeData;

// Chemical parameters
pub const CHEMICAL_SIZE: f32 = 20.;
pub const CHEMICAL_ENERGY: f32 = 10.;
pub const CHEMICAL_SPAWN_RATE: f32 = 20.;
pub const CHEMICAL_MAX_NUM: usize = 400;
pub const CHEMICAL_COLOUR: Color = Color::linear_rgba(0.5, 0.1, 0.1, 0.75);

#[derive(Component)]
pub struct Chemical {
    pub energy: f32,
}

#[derive(Resource)]
pub struct ChemicalTimer(pub Timer);

impl Default for ChemicalTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1. / CHEMICAL_SPAWN_RATE, TimerMode::Repeating))
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

impl QuadTreeData for Chemical {}
