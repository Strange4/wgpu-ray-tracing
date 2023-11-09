@group(0) @binding(0)
var output_color: texture_storage_2d<rgba8unorm, write>;

struct SharedStageUniform {
    size: vec2f,
}

@group(1) @binding(0)
var<uniform> shared_stage_uniform: SharedStageUniform;

@compute
@workgroup_size(16,16,1)
fn compute_main(@builtin(global_invocation_id) compute_id: vec3u) {
    let screen_position = compute_id.xy;
    let dimentions = textureDimensions(output_color);
    // this is commented out for debugging
    // textureStore(output_color, screen_position, vec4f((vec2f(screen_position) / vec2f(dimentions)), 0.0, 1.0));
    textureStore(output_color, screen_position, get_pixel_color(screen_position));
}


fn get_pixel_color(screen_position: vec2u) -> vec4f {

    // the camera stuff
    let image_width = shared_stage_uniform.size.x;
    let image_height = shared_stage_uniform.size.y;

    let aspect_ratio = image_width / image_height;


    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;

    let camera_center = vec3f();
    let focal_length = 1.0;

    let viewport_u = vec3f(viewport_width, 0.0, 0.0);
    let viewport_v = vec3f(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / f32(image_width);
    let pixel_delta_v = viewport_v / f32(image_height);

    let viewport_upper_left = camera_center - vec3f(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixel_center = pixel_00 + (f32(screen_position.x) * pixel_delta_u) + (f32(screen_position.y) * pixel_delta_v);

    let ray_direction = pixel_center - camera_center;

    let ray = Ray(camera_center, ray_direction);
    return vec4f(get_ray_color(ray), 1.0);
}

const main_sphere = Sphere(0.5, vec3f(0.0, 0.0, -1.0));

fn get_ray_color(ray: Ray) -> vec3f {
    let distance = hit_sphere(main_sphere, ray);
    if(distance > 0.0) {
        let normal = sphere_point_normal(ray_at_distance(ray, distance), main_sphere.center);
        return 0.5 * (normal + 1.0);
    }

    let unit = normalize(ray.direction);
    let a = 0.5 * (unit.y + 1.0);
    return vec3f((1.0 - a) * vec3f(1.0) + a * vec3f(0.5, 0.7, 1.0));
}




// Ray part of the code

struct Ray {
    origin: vec3f,
    direction: vec3f,
}

fn length_squared(vector: vec3f) -> f32 {
    return vector.x * vector.x + vector.y * vector.y + vector.z * vector.z;
}

fn ray_at_distance(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.direction;
}


// Sphere part of the code
struct Sphere {
    radius: f32,
    center: vec3f,
}

fn hit_sphere(sphere: Sphere, ray: Ray) -> f32 {
    let origin_center = ray.origin - sphere.center;
    let a = length_squared(ray.direction);
    let half_b = dot(origin_center, ray.direction);
    let c = length_squared(origin_center) - sphere.radius * sphere.radius;
    let discriminant = half_b*half_b - a * c;
    if(discriminant < 0.0) {
        return -1.0;
    }
    return (-half_b - sqrt(discriminant)) / a;
}

// point is the point along the surface of the sphere that hit the sphere
// center is the center of the sphere
fn sphere_point_normal(point: vec3f, center: vec3f) -> vec3f {
    return normalize(point - center);
}