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
    @location(0) base_colour: vec4<f32>,
    @location(1) model_position: vec4<f32>,
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
    let depth = model.depth.z * model.depth.z;
    out.base_colour = vec4<f32>(0.1 * depth, 0.1 * depth, 0.5 * depth, 1.0);
    out.model_position = vec4<f32>(model.position, 1.0);
    out.clip_position = (camera.view_proj * out.model_position).xyww;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if(in.model_position.z > 0.9) {
        let dist = vec2<f32>(in.model_position.x, in.model_position.y);
        let x2y2 = 2 - (dist.x * dist.x) + (dist.y * dist.y);
        return vec4<f32>(0.5 * x2y2, 0.0, 0.0, 1.0);
    }
    return in.base_colour;
}
