struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) depth: vec3<f32>,
};

struct InstanceInput {
    @location(4) a: vec4<f32>,
    @location(5) b: vec4<f32>,
    @location(6) c: vec4<f32>,
    @location(7) d: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) base_colour: vec3<f32>,
    @location(1) model_position: vec4<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

// Vertex shader

@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(1) @binding(0)
var wave_tex: texture_2d<f32>;
@group(1) @binding(1)
var wave_sampler: sampler;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    let depth = model.depth.z;
    out.base_colour = vec3<f32>(depth, depth, depth);
    out.model_position = vec4<f32>(model.position, depth);
    out.clip_position = (camera.view_proj * out.model_position).xyww;
    return out;
}

// Fragment shader

fn arsinh(x: f32) -> f32 {
    return log(x + sqrt(1 + x*x));
}

fn wave(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    return 0.6*sin(a*x) + 0.4*cos(b*x) + 0.2*sin(c*x) + 0.1*cos(d*x);
}

fn wave_intensity(theta: f32) -> f32 {
    return 3.0 * pow(theta - 0.5, 2.0) - 2.0 * pow(theta - 0.5, 3.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let surface_colour = vec3<f32>(0.1, 0.5, 0.8);
    let deep_colour = vec3<f32>(0.0, 0.005, 0.03);
    let adjacent = length(in.model_position.xy);
    let theta = atan(in.model_position.z / max(0.01, adjacent));
    let depth = max((0.1559718442300574 * arsinh(16.0*(theta - 0.75))) + 0.67, 0.1);
    var base_colour = deep_colour + depth * (surface_colour - deep_colour);
    let wave_theta = min(1.5, max(theta, 0.5));
    base_colour = base_colour + 
    (
        f32(in.model_position.z > 0.0) * 
        vec3<f32>(0.15) * 
        pow(cos(1.5*in.model_position.x), 12.0) *
        pow(cos(1.5*in.model_position.y), 12.0)
    ) +
    (
        vec3<f32>(0.3) * 
        wave_intensity(wave_theta) * 
        textureSample(wave_tex, wave_sampler, vec2<f32>(0.5*(in.model_position.x+1.0), 0.5*(in.model_position.y+1.0))).x
    );
    return vec4<f32>(base_colour, 1.0);
}
