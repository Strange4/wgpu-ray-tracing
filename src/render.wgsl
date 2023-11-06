struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) texture_coordinates: vec2f,
}

var<private> v_positions: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(-1.0, -1.0),
);
var<private> vertex_positions: array<vec2f, 6> =  array<vec2f, 6>(
    // bottom right triangle
    vec2f(1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, -1.0),

    // top left triangle
    vec2f(1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(-1.0, 1.0),
);

var<private> texture_for_vertex: array<vec2f, 6> = array<vec2f, 6>(
    // bottom right triangle
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 1.0),

    // top left triangle
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(0.0, 0.0),
);
@vertex
fn vertex_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4f(vertex_positions[index], 0.0, 1.0);
    output.texture_coordinates = texture_for_vertex[index];
    return output;
}

@group(0) @binding(0)
var texture_to_render: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
    return textureSample(texture_to_render, texture_sampler, input.texture_coordinates);
}