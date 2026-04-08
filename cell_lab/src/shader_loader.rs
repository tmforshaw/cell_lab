use bevy::{
    asset::{load_internal_asset, uuid::Uuid},
    prelude::*,
};

pub struct ShaderLoaderPlugin;

// Load the shaders into Bevy, allowing them to be used as imports (Path relative to src/ folder)
impl Plugin for ShaderLoaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            Handle::from(Uuid::new_v4()),
            "../assets/shaders/hsv_utils.wgsl",
            Shader::from_wgsl
        );
    }
}
