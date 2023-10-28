struct VertexOut {
    @location(0) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
}

struct TriangleInfo {
    angle: f32,
}

@group(0) @binding(0)
var<uniform> traingle_info: TriangleInfo;

var<private> vertex_positions: array<vec2f, 3> = array<vec2f, 3> (
    vec2f(0.0, 1.0),
    vec2f(1.0, -1.0),
    vec2f(-1.0, -1.0),
);

var<private> vertex_colors: array<vec4f, 3> = array<vec4f, 3> (
    vec4f(1.0, 0.0, 0.0, 1.0),
    vec4f(0.0, 1.0, 0.0, 1.0),
    vec4f(0.0, 0.0, 1.0, 1.0),
);

@vertex
fn vertex_main(@builtin(vertex_index) index: u32) -> VertexOut {
    var out: VertexOut;

    out.position = vec4f(vertex_positions[index], 0.0, 1.0);
    out.position.x *= cos(traingle_info.angle);
    out.color = vertex_colors[index];

    return out;
}

@fragment
fn fragment_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}