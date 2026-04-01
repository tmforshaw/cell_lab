#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> colour: vec4<f32>;

fn sine_wave_radius(pos: vec2<f32>, frequency: f32, amplitude: f32) -> f32 {
    return abs(sin(atan2(pos.y, pos.x) * frequency)) * amplitude;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Centre the uv coordinates and check the distance from the centre
    let centred = in.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let dist = length(centred);
    if dist > 1.0 { discard; }

    let cell_radius = 0.95;
    let nucleus_radius = 0.167;
    let border_thickness = 0.05;

    let nucleus_radius_modulation = sine_wave_radius(centred, 1.5, 0.01) + nucleus_radius;
    let cell_radius_modulation = sine_wave_radius(centred, 4.0, 1.0 - cell_radius - border_thickness) + cell_radius; // No modulation of cell radius when border_thickness + cell_radius == 1.0

    let nucleus_colour = vec4<f32>(colour.rgb * 0.2, 1.0);
    let border_colour = vec4<f32>(colour.rgb * 0.33, 1.0);

    if dist > cell_radius_modulation + border_thickness {
        // Outside of Border
        discard;
    } else if dist > cell_radius_modulation {
        // Border
        return border_colour;
    } else if dist < nucleus_radius_modulation {
        // Nucleus
        return nucleus_colour;
    } else {
        // Cell
        return vec4<f32>(colour.rgb, 1.0);
    }
}
