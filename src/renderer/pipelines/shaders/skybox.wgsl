// Vertex shader

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

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.position;
    out.clip_position = vec4<f32>(model.position, 0.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
