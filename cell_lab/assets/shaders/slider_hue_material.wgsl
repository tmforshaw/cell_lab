#import bevy_ui::ui_vertex_output::UiVertexOutput;

#import colour_picker::hsv_utils::hsv_to_rgb;

@group(1) @binding(0)
var<uniform> border_size: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Don't draw outside of the borders
    if uv.x < border_size.x || uv.x > 1.0 - border_size.y {
        discard;
    }

    // Scale the UV coords to remove the border
    let uv_scaled = uv.x / (1.0 - border_size.x - border_size.y);

    // Horizontal gradient from 0° → 360°
    let hue = uv_scaled * 360.0;

    return vec4<f32>(hsv_to_rgb(hue, 1.0, 1.0), 1.0);
}