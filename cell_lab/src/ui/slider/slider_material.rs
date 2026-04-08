use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct SliderHueMaterial {
    #[uniform(0)]
    pub border_size: Vec4,
}

impl UiMaterial for SliderHueMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/slider_hue_material.wgsl".into()
    }
}
