@group(0) @binding(0)
var output_color: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16,16,1)
fn compute_main(@builtin(global_invocation_id) compute_id: vec3u) {
    let screen_position = compute_id.xy;
    let dimentions = textureDimensions(output_color);
    textureStore(output_color, screen_position, vec4f(vec2f(screen_position) / vec2f(dimentions), 1.0, 1.0));
}