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
const SHARK_VERTICES: &[Vertex] = &[
    // nose
    Vertex{position: [-2.2765,  0.0000,  0.0475], normal: [-1.0, 0.0, 0.0]},
    // back of head
    Vertex{position: [ 0.0000,  0.0000,  1.4142], normal: [ 0.0,  0.0,  1.0]}, // T
    Vertex{position: [ 0.0000,  1.3181,  0.0000], normal: [ 0.0,  1.0,  0.0]}, // R
    Vertex{position: [ 0.0000,  0.0000, -0.8000], normal: [ 0.0,  0.0, -1.0]}, // B
    Vertex{position: [ 0.0000, -1.3181,  0.0000], normal: [ 0.0, -1.0,  0.0]}, // L
    // tail base
    Vertex{position: [ 5.1993,  0.0000,  0.4807], normal: [ 0.3338,  0.0000,  0.942643]}, // T
    Vertex{position: [ 5.1925,  0.5398,  0.0000], normal: [ 0.3333,  0.9428,  0.003400]}, // R
    Vertex{position: [ 5.1993,  0.0000, -0.4950], normal: [ 0.3031,  0.0000, -0.952969]}, // B
    Vertex{position: [ 5.1925, -0.5398,  0.0000], normal: [ 0.3333, -0.9428,  0.003400]}, // L
    Vertex{position: [ 6.0382,  0.0000,  0.0000], normal: [ 0.4557,  0.8901,  0.002100]}, // tip R
    Vertex{position: [ 6.0382,  0.0000,  0.0000], normal: [ 0.4557, -0.8901,  0.002100]}, // tip L
    // tail fin right side
    Vertex{position: [ 6.2581,  0.0000, -1.7362], normal: [0.0, 1.0, 0.0]}, // bottom
    Vertex{position: [ 5.1993,  0.0000, -0.4950], normal: [0.0, 1.0, 0.0]}, // bottom root
    Vertex{position: [ 6.3173,  0.0000,  2.4552], normal: [0.0, 1.0, 0.0]}, // top 
    Vertex{position: [ 5.1993,  0.0000,  0.4807], normal: [0.0, 1.0, 0.0]}, // top root
    // tail fin left side
    Vertex{position: [ 6.2581,  0.0000, -1.7362], normal: [0.0, -1.0, 0.0]}, // bottom
    Vertex{position: [ 5.1993,  0.0000, -0.4950], normal: [0.0, -1.0, 0.0]}, // bottom root
    Vertex{position: [ 6.3173,  0.0000,  2.4552], normal: [0.0, -1.0, 0.0]}, // top 
    Vertex{position: [ 5.1993,  0.0000,  0.4807], normal: [0.0, -1.0, 0.0]}, // top root
    // dorsal fin right side
    Vertex{position: [ 1.2823,  0.0000,  1.1000], normal: [0.0,  1.0, 0.0]},
    Vertex{position: [ 3.0444,  0.0000,  2.2953], normal: [0.0,  1.0, 0.0]},
    Vertex{position: [ 2.9776,  0.0000,  0.8500], normal: [0.0,  1.0, 0.0]},
    // dorsal fin left side
    Vertex{position: [ 1.2823,  0.0000,  1.1000], normal: [0.0, -1.0, 0.0]},
    Vertex{position: [ 3.0444,  0.0000,  2.2953], normal: [0.0, -1.0, 0.0]},
    Vertex{position: [ 2.9776,  0.0000,  0.8500], normal: [0.0, -1.0, 0.0]},
    // right fin upper
    Vertex{position: [ 1.4013,  1.1080,  0.0000], normal: [ 0.1301,  0.4270,  0.8948]}, // root
    Vertex{position: [ 1.9136,  3.1625, -0.3939], normal: [-0.0272,  0.1817,  0.9830]}, // tip
    // right fin lower
    Vertex{position: [ 1.4013,  1.1080,  0.0000], normal: [-0.1301, -0.4270, -0.8948]}, // root
    Vertex{position: [ 1.9136,  3.1625, -0.3939], normal: [ 0.0272, -0.1817, -0.9830]}, // tip
    // left fin upper
    Vertex{position: [ 1.4013, -1.1080,  0.0000], normal: [ 0.1301,  0.4270,  0.8948]}, // root
    Vertex{position: [ 1.9136, -3.1625, -0.3939], normal: [-0.0272,  0.1817,  0.9830]}, // tip
    // left fin lower
    Vertex{position: [ 1.4013, -1.1080,  0.0000], normal: [-0.1301, -0.4270, -0.8948]}, // root
    Vertex{position: [ 1.9136, -3.1625, -0.3939], normal: [ 0.0272, -0.1817, -0.9830]}, // tip
];

#[rustfmt::skip]
const SHARK_INDICES: &[u16] = &[
    // head
    0, 1, 2,  0, 2, 3,  0, 3, 4,  0, 4, 1,
    // body
    1, 5, 2,   2, 5, 6,
    2, 6, 3,   3, 6, 7,
    3, 7, 4,   4, 7, 8,
    4, 8, 1,   1, 8, 5,
    5, 9, 6,   6, 9, 7,
    5, 8, 10,  8, 7, 10,
    // tail R
    11, 12, 9,   14, 13, 9,
    // tail L
    16, 15, 10,  17, 18, 10,
    // dorsal
    19, 20, 21,  22, 24, 23,
    // right fin
    25, 26, 2,   2,  28, 27,
    // left fin
    30, 29, 4,   4,  31, 32,
];

const SKYBOX_INSTANCES: &[InstanceRaw] = &[InstanceRaw {
    model: [
        [0.2, 0.0, 0.0, 0.0],
        [0.0, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.2, 0.0],
        [0.0, 1.0, 0.0, 0.2],
    ],
}];

pub struct SharkPipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u16,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
    camera_uniforms: CameraUniformBuffer,
}

impl Pipeline for SharkPipeline {
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

impl SharkPipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> SharkPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shark Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shark.wgsl").into()),
        });

        let camera_uniforms = CameraUniformBuffer::new(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shark Render Pipeline Layout"),
                bind_group_layouts: &[camera_uniforms.bind_group_layout()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shark Render Pipeline"),
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
            label: Some("Shark Vertex Buffer"),
            contents: bytemuck::cast_slice(SHARK_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shark Index Buffer"),
            contents: bytemuck::cast_slice(SHARK_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        // instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shark Instance Buffer"),
            contents: bytemuck::cast_slice(SKYBOX_INSTANCES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // texture buffer

        SharkPipeline {
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: SHARK_VERTICES.len() as u16,
            index_buffer: index_buffer,
            num_indices: SHARK_INDICES.len() as u32,
            instance_buffer: instance_buffer,
            num_instances: SKYBOX_INSTANCES.len() as u32,
            camera_uniforms: camera_uniforms,
        }
    }
}
