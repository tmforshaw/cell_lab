#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> colour: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Centre the uv coordinates and check the distance from the centre
    let centred = in.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let dist = length(centred);
    if dist > 1.0 { discard; }

    let border_percent = 0.95;
    let border_colour_mult = 0.075;

    let inner_circle = 0.15;

    if dist > border_percent || dist < inner_circle{
        return vec4<f32>(colour.rgb * border_colour_mult, 1.0);
    } else {
        return vec4<f32>(colour.rgb, 1.0);
    }

}
