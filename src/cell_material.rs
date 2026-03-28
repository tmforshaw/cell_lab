use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CellMaterial {
    #[uniform(0)]
    pub colour: Vec4,
}

impl Material2d for CellMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cell_material.wgsl".into()
    }
}

impl CellMaterial {
    #[must_use]
    pub fn new(colour: Color) -> Self {
        Self {
            colour: colour.to_linear().to_vec4(),
        }
    }
}
