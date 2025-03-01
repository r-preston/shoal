struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) unused: vec3<f32>,
};

struct InstanceInput {
    @location(4) a: vec4<f32>,
    @location(5) b: vec4<f32>,
    @location(6) c: vec4<f32>,
    @location(7) d: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) model_position: vec3<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};


// Vertex shader

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.model_position = model.position;
    out.clip_position = (camera.view_proj * vec4<f32>(out.model_position, 1.0)).xyww;
    return out;
}


// Fragment shader

@group(1) @binding(0)
var wave_tex: texture_2d<f32>;
@group(1) @binding(1)
var wave_sampler: sampler;

fn arsinh(x: f32) -> f32 {
    return log(x + sqrt(1 + x*x));
}

fn wave_intensity(theta: f32) -> f32 {
    return 3.0 * pow(theta - 0.5, 2.0) - 2.0 * pow(theta - 0.5, 3.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // constants
    let surface_colour = vec3<f32>(0.1, 0.5, 0.8);
    let deep_colour = vec3<f32>(0.0, 0.005, 0.03);

    // calculate polar angle of fragment from skybox centre
    let adjacent = length(in.model_position.xy);
    let theta = atan(in.model_position.z / max(0.01, adjacent));
    // reconstruct a depth value that increases smoothly over surface of skybox 'sphere' to avoid corners showing
    let depth = max((0.1559718442300574 * arsinh(16.0*(theta - 0.75))) + 0.67, 0.1);
    // fades out wave texture away from surface
    let wave_theta = min(1.5, max(theta, 0.5));

    let colour =  
    (
        // base colour based on water depth
        deep_colour + depth * (surface_colour - deep_colour)
    ) +
    (
        // simulate sun overhead
        f32(in.model_position.z > 0.0) * 
        vec3<f32>(0.2) * 
        pow(cos(1.5*in.model_position.x), 12.0) *
        pow(cos(1.5*in.model_position.y), 12.0)
    ) +
    (
        // create wave pattern on the surface
        vec3<f32>(0.2) * 
        wave_intensity(wave_theta) * 
        textureSample(wave_tex, wave_sampler, vec2<f32>(0.5*(in.model_position.x+1.0), 0.5*(in.model_position.y+1.0))).x
    );

    return vec4<f32>(colour, 1.0);
}
