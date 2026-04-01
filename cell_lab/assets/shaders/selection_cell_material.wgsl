#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> colour: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Centre the uv coordinates and check the distance from the centre
    let centred = in.uv * 2.0 - vec2<f32>(1.0, 1.0);
    let dist = length(centred);
    if dist > 1.0 { discard; }

    return colour;
}

