use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CellMaterial {
    #[uniform(0)]
    pub colour: Vec4,

    #[uniform(1)]
    pub show_cell_info: u32,

    #[uniform(2)]
    pub split_angle: f32,

    #[uniform(3)]
    pub split_fraction: f32,
}

impl Material2d for CellMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cell_material.wgsl".into()
    }
}

impl CellMaterial {
    #[must_use]
    pub fn new(colour: Color, show_cell_info: bool, split_angle: f32, split_fraction: f32) -> Self {
        Self {
            colour: colour.to_linear().to_vec4(),
            show_cell_info: show_cell_info as u32,
            split_angle,
            split_fraction,
        }
    }
}
