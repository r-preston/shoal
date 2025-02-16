use crate::renderer::{pipelines::Pipeline, Renderer};
use crate::utility::Vec4;
use crate::world::World;
use wgpu::{Device, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

pub struct SkyboxPipeline {
    render_pipeline: wgpu::RenderPipeline,
    skybox: Box<[Vec4]>,
}

impl Pipeline for SkyboxPipeline {
    fn geometry(&self, world: &World) -> &Box<[Vec4]> {
        &self.skybox
    }

    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
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

        SkyboxPipeline {
            render_pipeline: render_pipeline,
            skybox: Self::skybox(),
        }
    }

    #[rustfmt::skip]
    fn skybox() -> Box<[Vec4]> {
        Box::new([
            // -z
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0, -1.0, -1.0 ,0.0),    Vec4::new( 1.0,  1.0, -1.0, 0.0),
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0,  1.0, -1.0, 0.0),    Vec4::new(-1.0,  1.0, -1.0, 0.0),
            // +z
            Vec4::new(-1.0, -1.0,  1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0),    Vec4::new( 1.0, -1.0,  1.0, 0.0),
            Vec4::new(-1.0, -1.0,  1.0, 0.0),    Vec4::new(-1.0,  1.0,  1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0),
            // -y
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0, -1.0,  1.0, 0.0),    Vec4::new( 1.0, -1.0, -1.0, 0.0),
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0, -1.0,  1.0, 0.0),    Vec4::new( 1.0, -1.0,  1.0, 0.0),
            // +y
            Vec4::new(-1.0,  1.0, -1.0, 0.0),    Vec4::new( 1.0,  1.0, -1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0),
            Vec4::new(-1.0,  1.0, -1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0),    Vec4::new(-1.0,  1.0,  1.0, 0.0),
            // -x
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new(-1.0,  1.0, -1.0, 0.0),    Vec4::new(-1.0,  1.0,  1.0, 0.0),
            Vec4::new(-1.0, -1.0, -1.0, 0.0),    Vec4::new(-1.0,  1.0,  1.0, 0.0),    Vec4::new(-1.0, -1.0,  1.0, 0.0),
            // +x
            Vec4::new( 1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0),    Vec4::new( 1.0,  1.0, -1.0, 0.0),
            Vec4::new( 1.0, -1.0, -1.0, 0.0),    Vec4::new( 1.0, -1.0,  1.0, 0.0),    Vec4::new( 1.0,  1.0,  1.0, 0.0)
        ])
    }
}
