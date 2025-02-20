use crate::renderer::{pipelines::Pipeline, Renderer};
use crate::utility::{Position, Vertex};
use crate::world::World;
use wgpu::{Device, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

#[rustfmt::skip]
const SKYBOX_VERTICES: &[Vertex] = &[
    Vertex{position: Position{x: 1.0,  y: 1.0,  z: 1.0}},
    Vertex{position: Position{x: 1.0,  y: 1.0,  z:-1.0}},
    Vertex{position: Position{x: 1.0,  y:-1.0,  z: 1.0}},
    Vertex{position: Position{x: 1.0,  y:-1.0,  z:-1.0}},
    Vertex{position: Position{x:-1.0,  y: 1.0,  z: 1.0}},
    Vertex{position: Position{x:-1.0,  y: 1.0,  z:-1.0}},
    Vertex{position: Position{x:-1.0,  y:-1.0,  z: 1.0}},
    Vertex{position: Position{x:-1.0,  y:-1.0,  z:-1.0}}
];

#[rustfmt::skip]
const SKYBOX_INDICES: &[u16] = &[];

pub struct SkyboxPipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u16,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
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
}

impl SkyboxPipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> SkyboxPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // describe vertex shader attachments
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
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

        SkyboxPipeline {
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: SKYBOX_VERTICES.len() as u16,
            index_buffer: index_buffer,
            num_indices: SKYBOX_INDICES.len() as u32,
            instance_buffer: instance_buffer,
            num_instances: 0,
        }
    }
}
