use crate::renderer::camera::{Camera, CameraUniformBuffer};
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
const FISH_VERTICES: &[Vertex] = &[
    // RHS
    Vertex{position: [ 1.0,  0.0,  0.0 ], normal: [ 0.196116, -0.980581, 0.0      ]},
    Vertex{position: [ 0.5,  0.0,  0.25], normal: [ 0.0,      -0.980581, 0.196116 ]},
    Vertex{position: [ 0.5,  0.0, -0.25], normal: [ 0.0,      -0.980581, -0.196116]},
    Vertex{position: [-0.75, 0.0,  0.0 ], normal: [-0.196116, -0.980581, 0.0      ]},
    Vertex{position: [-1.0,  0.0,  0.35], normal: [ 0.0,      -1.0,      0.0      ]},
    Vertex{position: [-0.88, 0.0, -0.3 ], normal: [ 0.0,      -1.0,      0.0      ]},
    // LHS
    Vertex{position: [ 1.0,  0.0,  0.0 ], normal: [ 0.196116,  0.980581, 0.0      ]},
    Vertex{position: [ 0.5,  0.0,  0.25], normal: [ 0.0,       0.980581, 0.196116 ]},
    Vertex{position: [ 0.5,  0.0, -0.25], normal: [ 0.0,       0.980581, -0.196116]},
    Vertex{position: [-0.75, 0.0,  0.0 ], normal: [-0.196116,  0.980581, 0.0      ]},
    Vertex{position: [-1.0,  0.0,  0.35], normal: [ 0.0,       1.0,      0.0      ]},
    Vertex{position: [-0.88, 0.0, -0.3 ], normal: [ 0.0,       1.0,      0.0      ]},
];

#[rustfmt::skip]
const FISH_INDICES: &[u16] = &[    
    0, 1, 2,   1, 3, 2,   3, 4,  5,  // RHS
    6, 8, 7,   7, 8, 9,   9, 11, 10, // LHS
];

const SKYBOX_INSTANCES: &[InstanceRaw] = &[InstanceRaw {
    model: [
        [1.0, 0.1, 0.0, 0.0],
        [0.0, 1.0, 0.3, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
}];

pub struct FishPipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u16,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
    camera_uniforms: CameraUniformBuffer,
}

impl Pipeline for FishPipeline {
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
        let camera_matrix = camera.projection_matrix() * camera.view_matrix();
        self.camera_uniforms
            .update(queue, &camera_matrix.into(), &camera.position().into());
    }
    fn bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        vec![self.camera_uniforms.bind_group()]
    }
    fn camera(&self) -> &CameraUniformBuffer {
        &self.camera_uniforms
    }
}

impl FishPipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> FishPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fish Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/fish.wgsl").into()),
        });

        let camera_uniforms = CameraUniformBuffer::new(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Fish Render Pipeline Layout"),
                bind_group_layouts: &[camera_uniforms.bind_group_layout()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Fish Render Pipeline"),
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
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
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
            label: Some("Fish Vertex Buffer"),
            contents: bytemuck::cast_slice(FISH_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fish Index Buffer"),
            contents: bytemuck::cast_slice(FISH_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        // instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fish Instance Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // texture buffer

        FishPipeline {
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: FISH_VERTICES.len() as u16,
            index_buffer: index_buffer,
            num_indices: FISH_INDICES.len() as u32,
            instance_buffer: instance_buffer,
            num_instances: SKYBOX_INSTANCES.len() as u32,
            camera_uniforms: camera_uniforms,
        }
    }
}
