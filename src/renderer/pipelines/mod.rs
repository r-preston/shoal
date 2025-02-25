pub mod skybox;

use wgpu::{Device, SurfaceConfiguration};

use crate::renderer::Renderer;
use crate::utility::{InstanceRaw, Position, Vertex};
use crate::world::World;

use super::camera::Camera;

const VERTEX_BUFFER_DESCRIPTOR: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    // We need to switch from using a step mode of Vertex to Instance
    // This means that our shaders will only change to use the next
    // instance when the shader starts processing a new instance
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &[wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x3,
    }],
};

const INSTANCE_BUFFER_DESCRIPTOR: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Instance,
    attributes: &[
        // represent mat4 as 4xvec4
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            shader_location: 2,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            shader_location: 3,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            shader_location: 4,
            format: wgpu::VertexFormat::Float32x4,
        },
    ],
};

pub struct Pipelines(Vec<Box<dyn Pipeline>>);

impl Pipelines {
    pub fn generate(device: &Device, config: &SurfaceConfiguration) -> Pipelines {
        let mut pipelines = Vec::<Box<dyn Pipeline>>::new();

        pipelines.push(Box::new(skybox::SkyboxPipeline::new(device, config)));

        Pipelines(pipelines)
    }

    pub fn pipelines(&self) -> &Vec<Box<dyn Pipeline>> {
        &self.0
    }

    pub fn mutable_pipelines(&mut self) -> &mut Vec<Box<dyn Pipeline>> {
        &mut self.0
    }
}
pub trait Pipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline;

    fn num_vertices(&self) -> u16;
    fn vertex_buffer(&self) -> &wgpu::Buffer;

    fn num_indices(&self) -> u32;
    fn index_buffer(&self) -> &wgpu::Buffer;

    fn num_instances(&self) -> u32;
    fn instance_buffer(&self) -> &wgpu::Buffer;

    fn bind_groups(&self) -> Vec<&wgpu::BindGroup>;

    fn update_camera(&mut self, queue: &wgpu::Queue, camera: &Camera);
    fn update_instances(&mut self, world: &World);
}
