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
    // return vec4<f32>(vertex_positions[index], 0.0, 1.0);
    return output;
}

@group(0) @binding(0)
var texture_to_render: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
    return textureSample(texture_to_render, texture_sampler, input.texture_coordinates);
    // return vec4f(1.0, 0.50, 0.3, 1.0);
}

fn drawLCD(fc: vec2f , num: f32) -> f32 {
	var v=1.;
	for(var i: f32 = 1.0 ; i<11.0; i += 1.0) {
	    var dig = i32((num/pow(10.,i) % 1.)*10.);
	    var uv: vec2f = (fc * 0.1) + vec2f(i*1.5, 0.);
		
	    var u= uv.y>=min(0. , abs(uv.x) - 0.5);
	    uv.y=abs(uv.y) - 0.5;
	    var c=false;

	    if(abs(uv.x) < abs(uv.y)){ 
            uv=uv.yx; 
            c=!c; 
        }

	    var l=(uv.x<0.);
	    uv.y=abs(uv.y) - 0.4;
	    uv.x=abs(abs(uv.x) - 0.5);
	
            dig-= (dig / 10)*10;
	
	    var val: f32;
	    if(((dig==0) && (c&&l)) ||
	       ((dig==1) && (c||l)) ||
	       ((dig==2) && (((u&&l)||!(u||l))&&!c)) ||
	       ((dig==3) && (l&&!c)) ||
	       ((dig==4) && ((c&&!l)||(l&&!u))) ||
	       ((dig==5) && (!c &&((!l&&u) || (l&&!u)))) ||
	       ((dig==6) && (u&&!c&&!l)) ||
	       ((dig==7) && ((l||c)&&!u)) ||
	       ((dig==9) && (!u&&l))){
             val= 1.;
             }
	    else {val= uv.x+max(0.,uv.y);}
		
	    v=min(v,val);
	}
	
	return smoothstep(0.06,0.05,v);
}