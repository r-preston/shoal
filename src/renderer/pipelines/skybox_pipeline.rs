use crate::renderer::camera::Camera;
use crate::renderer::uniform::UniformBuffer;
use crate::renderer::{
    pipelines::{texture::Texture, Pipeline},
    Renderer,
};
use crate::utility::{InstanceRaw, Mat3, Mat4, Position, Vertex};
use crate::world::World;
use wgpu::core::instance;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, Instance, Queue, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

use super::{INSTANCE_BUFFER_DESCRIPTOR, VERTEX_BUFFER_DESCRIPTOR};

#[rustfmt::skip]
const SKYBOX_VERTICES: &[Vertex] = &[
    Vertex{position: [ 1.0,  1.0,  1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [ 1.0,  1.0, -1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [ 1.0, -1.0,  1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [ 1.0, -1.0, -1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [-1.0,  1.0,  1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [-1.0,  1.0, -1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [-1.0, -1.0,  1.0], normal: [ 0.0, 0.0, 0.0]},
    Vertex{position: [-1.0, -1.0, -1.0], normal: [ 0.0, 0.0, 0.0]}
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
    uniforms: UniformBuffer,
    texture: Texture,
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
        let mut trunc_matrix = camera.view_matrix();
        // remove translational components of camera view
        trunc_matrix.x.w = 0.0;
        trunc_matrix.y.w = 0.0;
        trunc_matrix.z.w = 0.0;
        trunc_matrix.w.x = 0.0;
        trunc_matrix.w.y = 0.0;
        trunc_matrix.w.z = 0.0;
        trunc_matrix = camera.projection_matrix() * trunc_matrix;
        self.uniforms
            .update(queue, &trunc_matrix.into(), &camera.position().into());
    }
    fn bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        vec![
            self.uniforms.bind_group(),
            self.texture.bind_group.as_ref().unwrap(),
        ]
    }
}

impl SkyboxPipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration, queue: &Queue) -> SkyboxPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skybox Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
        });

        let uniforms = UniformBuffer::new(device);

        let texture = Texture::from_bytes(include_bytes!("textures/waves.png"), device, queue);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Skybox Render Pipeline Layout"),
                bind_group_layouts: &[
                    uniforms.bind_group_layout(),
                    texture.bind_group_layout.as_ref().unwrap(),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // describe vertex shader attachments
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[VERTEX_BUFFER_DESCRIPTOR, INSTANCE_BUFFER_DESCRIPTOR],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // describe fragment shader and screen colour attachments
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
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
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        // instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Instance Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // texture buffer

        SkyboxPipeline {
            render_pipeline,
            vertex_buffer,
            num_vertices: SKYBOX_VERTICES.len() as u16,
            index_buffer,
            num_indices: SKYBOX_INDICES.len() as u32,
            instance_buffer,
            num_instances: SKYBOX_INSTANCES.len() as u32,
            uniforms,
            texture,
        }
    }
}
