#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> colour: vec4<f32>;

@group(2) @binding(1)
var<uniform> show_cell_info: u32;

@group(2) @binding(2)
var<uniform> split_angle: f32;

@group(2) @binding(3)
var<uniform> split_fraction: f32;

fn sine_wave_radius(pos: vec2<f32>, frequency: f32, amplitude: f32) -> f32 {
    return abs(sin(atan2(pos.y, pos.x) * frequency)) * amplitude;
}

fn get_cell_with_nucleus(uv: vec2<f32>, dist: f32) -> vec4<f32> {
    let cell_radius = 0.95;
    let nucleus_radius = 0.167;
    let border_thickness = 0.05;

    let nucleus_radius_modulation = sine_wave_radius(uv, 1.5, 0.01) + nucleus_radius;
    let cell_radius_modulation = sine_wave_radius(uv, 4.0, 1.0 - cell_radius - border_thickness) + cell_radius; // No modulation of cell radius when border_thickness + cell_radius == 1.0

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

fn rotate(v: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);

    return vec2<f32>(c * v.x - s * v.y, s * v.x + c * v.y);
}


fn dashed_offset_rotated_line_mask(uv: vec2<f32>, angle: f32, offset: f32, segments: f32, gap_percent: f32, line_width: f32) -> f32 {
    // Rotate a vertical line by the angle, then get the normal to that
    let dir = rotate(vec2<f32>(0.0, 1.0), angle);
    let normal = vec2<f32>(-dir.y, dir.x);

    // Shift the UV by the offset percent (Along the normal dir)
    let shifted_uv = uv - normal * offset;

    // Parameterise the length of the dashed arrow
    let t = dot(shifted_uv, dir);
    let dist_to_center = dot(shifted_uv, normal);
    let max_len = sqrt(1.0 - dist_to_center * dist_to_center); // Calculate the maximum length based on the offset, and radius of 1.0

    let dist_to_line = abs(dot(shifted_uv, normal));

    // Get a line mask for the entire line
    let line_mask = step(dist_to_line, line_width);

    // Create a new mask to be used for the segments in that line mask
    let inside_segment = step(abs(t), max_len);

    let total_length = 2.0 * max_len;

    // Calculate the dash length and gap length to have N dashes within the line
    let dash_length = total_length * (1.0 - gap_percent) / (segments - gap_percent);
    let gap_length = dash_length * gap_percent / (1.0 - gap_percent);

    // Calculate the length of one cycle of the dash and gap
    let period = dash_length + gap_length;

    // Shift paramter t so it starts at one edge
    let t_shifted = t + max_len;

    // Repeat cycle of dash and gap
    let pattern_pos = fract(t_shifted / period) * period;

    // Create mask for all the dashes
    let dash_mask = step(pattern_pos, dash_length);

    // Combine all the masks
    return line_mask * dash_mask * inside_segment;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Centre the uv coordinates and check the distance from the centre
    let uv = in.uv * 2.0 - 1.0;
    let dist = length(uv);
    if dist > 1.0 { discard; }

    let cell_and_nucleus_colour = get_cell_with_nucleus(uv, dist);

    if show_cell_info == 1 {
        let line_colour =  vec4<f32>(1.0, 1.0, 1.0, 1.0);

        let line_segments = 6.0;
        let line_gap_percent = 0.3;
        let line_width = 0.06;

        let line_mask = dashed_offset_rotated_line_mask(uv, split_angle, 1.0 - split_fraction * 2.0, line_segments, line_gap_percent, line_width);

        return mix(cell_and_nucleus_colour, line_colour, line_mask);
    } else {
        return cell_and_nucleus_colour;
    }
}
