use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d};

use crate::{game::game_parameters::GameParameters, spatial_partitioning::quadtree::QuadTreeData};

#[derive(Component)]
pub struct Chemical {
    pub energy: f32,
}

#[derive(Resource)]
pub struct ChemicalTimer(pub Timer);

impl ChemicalTimer {
    #[must_use]
    pub fn new_from_parameters(param: &GameParameters) -> Self {
        Self(Timer::from_seconds(
            1. / param.simulation_mode.chemical_parameters.spawn_rate,
            TimerMode::Repeating,
        ))
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
