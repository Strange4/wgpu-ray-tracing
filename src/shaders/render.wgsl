struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) texture_coordinates: vec2f,
}

struct SharedStageUniform {
    size: vec2f,
}
@group(0) @binding(0)
var texture_to_render: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@group(1) @binding(0)
var<uniform> fragment_uniform: SharedStageUniform;

// This has to be outside the function or it shit's it's pants
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

@vertex
fn vertex_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    let texture_dimentions: vec2u = textureDimensions(texture_to_render);

    let texture_percentage_filled: vec2f = fragment_uniform.size / vec2f(texture_dimentions);

    var output: VertexOutput;
    output.clip_position = vec4f(vertex_positions[index], 0.0, 1.0);
    switch index {
        case 0u: {
            output.texture_coordinates = vec2f(texture_percentage_filled.x, 0.0);
        }
        case 1u {
            output.texture_coordinates = vec2f(0.0, texture_percentage_filled.y);
        }
        case 2u: {
            output.texture_coordinates = vec2f(texture_percentage_filled.x, texture_percentage_filled.y);
        }
        case 3u {
            output.texture_coordinates = vec2f(texture_percentage_filled.x, 0.0);
        }
        case 4u: {
            output.texture_coordinates = vec2f(0.0, texture_percentage_filled.y);
        }
        case 5u {
            output.texture_coordinates = vec2f(0.0, 0.0);
        }
        default: {}
    }
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
    return textureSample(texture_to_render, texture_sampler, input.texture_coordinates);
}