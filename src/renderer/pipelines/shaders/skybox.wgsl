struct VertexInput {
    @location(0) position: vec3<f32>
};

struct InstanceInput {
    @location(1) a: vec4<f32>,
    @location(2) b: vec4<f32>,
    @location(3) c: vec4<f32>,
    @location(4) d: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
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
    let no_translation_matrix = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0), 
        vec4<f32>(0.0, 1.0, 0.0, 0.0), 
        vec4<f32>(0.0, 0.0, 1.0, 0.0), 
        vec4<f32>(0.0, 0.0, 0.0, 0.0));
    var out: VertexOutput;
    out.color = model.position;//vec3<f32>(0.2, 1.0, 0.4);//model.position;
    out.clip_position =  camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
