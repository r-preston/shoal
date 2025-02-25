use crate::renderer::camera::{Camera, CameraUniformBuffer};
use crate::renderer::{pipelines::Pipeline, Renderer};
use crate::utility::{InstanceRaw, Position, Vertex};
use crate::world::World;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

use super::{INSTANCE_BUFFER_DESCRIPTOR, VERTEX_BUFFER_DESCRIPTOR};

#[rustfmt::skip]
const SKYBOX_VERTICES: &[Vertex] = &[
    Vertex{position: [ 1.0,  1.0,  1.0]},
    Vertex{position: [ 1.0,  1.0, -1.0]},
    Vertex{position: [ 1.0, -1.0,  1.0]},
    Vertex{position: [ 1.0, -1.0, -1.0]},
    Vertex{position: [-1.0,  1.0,  1.0]},
    Vertex{position: [-1.0,  1.0, -1.0]},
    Vertex{position: [-1.0, -1.0,  1.0]},
    Vertex{position: [-1.0, -1.0, -1.0]}
];

#[rustfmt::skip]
const SKYBOX_INDICES: &[u16] = &[
    1, 3, 2,   0, 1, 2, // +x face
    7, 5, 4,   7, 4, 6, // -x face
    1, 0, 5,   0, 4, 5, // +y face
    3, 7, 6,   3, 6, 2, // -y face
    0, 2, 6,   0, 6, 4, // +z face
    5, 7, 3,   5, 3, 1, // -z face
];

const SKYBOX_INSTANCES: &[InstanceRaw] = &[InstanceRaw {
    model: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
}];

pub struct SkyboxPipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u16,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
    camera_uniforms: CameraUniformBuffer,
}

impl Pipeline for SkyboxPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
    fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }
    fn num_vertices(&self) -> u16 {
        self.num_vertices
    }
    fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }
    fn num_indices(&self) -> u32 {
        self.num_indices
    }
    fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }
    fn num_instances(&self) -> u32 {
        self.num_instances
    }
    fn update_instances(&mut self, world: &World) {
        ()
    }
    fn update_camera(&mut self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera_uniforms.update(queue, camera);
    }
    fn bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        vec![self.camera_uniforms.bind_group()]
    }
}

impl SkyboxPipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> SkyboxPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
        });

        let camera_uniforms = CameraUniformBuffer::new(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera_uniforms.bind_group_layout()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // describe vertex shader attachments
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTEX_BUFFER_DESCRIPTOR, INSTANCE_BUFFER_DESCRIPTOR],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // describe fragment shader and screen colour attachments
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // describe data in vertex buffers
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        // instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        SkyboxPipeline {
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: SKYBOX_VERTICES.len() as u16,
            index_buffer: index_buffer,
            num_indices: SKYBOX_INDICES.len() as u32,
            instance_buffer: instance_buffer,
            num_instances: SKYBOX_INSTANCES.len() as u32,
            camera_uniforms: camera_uniforms,
        }
    }
}
