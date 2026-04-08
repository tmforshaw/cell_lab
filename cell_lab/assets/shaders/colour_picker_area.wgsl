#import bevy_ui::ui_vertex_output::UiVertexOutput;

#import colour_picker::hsv_utils::hsv_to_rgb;

@group(1) @binding(0)
var<uniform> hue: f32;

@group(1) @binding(1)
var<uniform> selected_uv: vec2<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let saturation = uv.x;
    let value = 1.0 - uv.y; // Make the bottom left the origin

    // Modify Saturation and Value to be closer to how they are perceived
    let perceptual_saturation = pow(uv.x, 0.9); // Compress white area
    let perceptual_value = pow(value, 2.2); // Expand dark area

    let hue_colour = hsv_to_rgb(hue, perceptual_saturation, perceptual_value);

    // Define properties about the selection
    let selection_radius = 0.02;
    let selection_border_radius = 0.005;
    let selection_colour = vec3<f32>(1.0, 1.0, 1.0);
    let selection_border = vec3<f32>(0.0, 0.0, 0.0);

    // Get the distance from the selected uv coord
    let dist_to_selection = length(uv - selected_uv);

    // Draw the selection, its border, or the background
    var final_colour_vec3: vec3<f32>;
    if dist_to_selection <= selection_radius {
        final_colour_vec3 = selection_colour;
    } else if dist_to_selection <= selection_radius + selection_border_radius {
        final_colour_vec3 = selection_border;
    } else {
        final_colour_vec3 = hue_colour;
    }

    return vec4<f32>(final_colour_vec3, 1.0);
}
