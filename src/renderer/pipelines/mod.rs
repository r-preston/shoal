mod fish_pipeline;
mod shark_pipeline;
mod skybox_pipeline;
pub mod texture;

use fish_pipeline::FishPipeline;
use shark_pipeline::SharkPipeline;
use skybox_pipeline::SkyboxPipeline;
use wgpu::{Device, Queue, SurfaceConfiguration};

use crate::utility::{InstanceRaw, Vertex};
use crate::world::World;
use super::camera::Camera;

const VERTEX_BUFFER_DESCRIPTOR: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &[
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x3,
        },
    ],
};

const INSTANCE_BUFFER_DESCRIPTOR: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Instance,
    attributes: &[
        // represent mat4 as 4xvec4
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 4,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            shader_location: 5,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            shader_location: 6,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            shader_location: 7,
            format: wgpu::VertexFormat::Float32x4,
        },
    ],
};

pub struct Pipelines(Vec<Box<dyn Pipeline>>);

impl Pipelines {
    pub fn generate(device: &Device, config: &SurfaceConfiguration, queue: &Queue) -> Pipelines {
        let mut pipelines = Vec::<Box<dyn Pipeline>>::new();

        pipelines.push(Box::new(FishPipeline::new(device, config)));
        pipelines.push(Box::new(SharkPipeline::new(device, config)));
        pipelines.push(Box::new(SkyboxPipeline::new(device, config, queue)));

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

    fn vertex_buffer(&self) -> &wgpu::Buffer;

    fn num_indices(&self) -> u32;
    fn index_buffer(&self) -> &wgpu::Buffer;

    fn num_instances(&self) -> u32;
    fn instance_buffer(&self) -> &wgpu::Buffer;

    fn bind_groups(&self) -> Vec<&wgpu::BindGroup>;

    fn update_camera(&mut self, queue: &wgpu::Queue, camera: &Camera);
    fn update_instances(&mut self, device: &Device, queue: &Queue, world: &World);

    //fn camera(&self) -> &UniformBuffer;
}
