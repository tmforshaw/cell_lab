use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ColourPickerMaterial {
    #[uniform(0)]
    pub hue: f32,

    #[uniform(1)]
    pub selected_uv: Vec2,
}

impl UiMaterial for ColourPickerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/colour_picker_area.wgsl".into()
    }
}
