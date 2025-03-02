struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct InstanceInput {
    @location(4) a: vec4<f32>,
    @location(5) b: vec4<f32>,
    @location(6) c: vec4<f32>,
    @location(7) d: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    pos: vec3<f32>,
};


// Vertex shader

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(instance.a, instance.b, instance.c, instance.d);
    var out: VertexOutput;
    out.world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_normal = normalize((model_matrix * vec4<f32>(model.normal, 0.0)).xyz);
    out.clip_position = camera.view_proj * out.world_position;
    return out;
}


// Fragment shader

fn step(x: f32) -> f32 {
    let a_gt_0 = f32(x > 0.0); 
    let a_lt_1 = f32(x < 1.0);
    return a_lt_1 * (1.0 - a_gt_0 * (3.0*x*x - 2.0*x*x*x));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_colour = vec3<f32>(0.5, 0.8, 1.0);
    let light_direction = vec3<f32>(0.146735, 0.146735, 0.978232);

    let z_angle = dot(in.world_normal, vec3<f32>(0.0, 0.0, 1.0));
    let base_colour = vec3<f32>(0.2 + 0.8*step(3.0 * z_angle + 0.75));

    let ambient_light = light_colour * 0.15;

    let diffuse_light = light_colour * 0.2 * max(dot(in.world_normal, light_direction), 0.0);

    return vec4<f32>(base_colour * (ambient_light + diffuse_light), 1.0);
}
