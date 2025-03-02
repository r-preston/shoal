mod camera;
mod pipelines;
mod uniform;

use crate::{
    utility::{Degrees, Mat4, Radians},
    World,
};
use camera::Camera;
use pipelines::{texture::Texture, Pipelines};
use wgpu::Buffer;
use winit::{event::*, window::Window};

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    camera: Camera,
    depth_texture: Texture,
    pipelines: Pipelines,
    mouse_pos: (f32, f32),
}

impl<'a> Renderer<'a> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'a Window, world: &World) -> Renderer<'a> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance_descriptor = wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::VULKAN,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        };
        let instance = wgpu::Instance::new(&instance_descriptor);

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let depth_texture = Texture::create_depth_texture(&device, &config);

        let pipelines = Pipelines::generate(&device, &config, &queue);

        let camera = Camera::new(world, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            camera,
            depth_texture,
            pipelines,
            mouse_pos: (0.0, 0.0),
        }
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera
                .set_aspect(self.config.width as f32 / self.config.height as f32);
            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved {
                position: mouse_position,
                ..
            } => {
                self.mouse_pos = (
                    1.0 - 2.0 * (mouse_position.x as f32 / self.config.width as f32),
                    2.0 * (mouse_position.y as f32 / self.config.height as f32) - 1.0,
                );
                return true;
            }

            WindowEvent::MouseWheel {
                delta: scroll_delta,
                ..
            } => {
                match scroll_delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.camera.move_in_out(-y);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.camera.move_in_out(-pos.y as f32);
                    }
                }
                return true;
            }

            _ => {
                // event not handled here
                return false;
            }
        };
    }

    pub fn update(&mut self, world: &mut World) {
        // update camera.
        // move camera. if cursor moves out of deadzone, move proportionally to distance outside deadzone
        let deadzone = (0.25f32, 0.99f32); // inner and outer edges from center
        let outside_deadzone = |x: f32| -> bool { x.abs() > deadzone.0 && x.abs() < deadzone.1 };
        let distance_beyond_deadzone =
            |x: f32| -> f32 { x.signum() * (x.abs() - deadzone.0).max(0.0) };
        if outside_deadzone(self.mouse_pos.0) {
            self.camera
                .move_left_right(distance_beyond_deadzone(self.mouse_pos.0));
        }
        if outside_deadzone(self.mouse_pos.1) {
            self.camera
                .move_up_down(distance_beyond_deadzone(self.mouse_pos.1));
        }
        // update buffers
        for pipeline in self.pipelines.mutable_pipelines() {
            pipeline.update_instances(world);
            pipeline.update_camera(&self.queue, &self.camera);
        }
    }

    pub fn render(&mut self, world: &World) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // queue for render commands
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for pipeline in self.pipelines.pipelines() {
                render_pass.set_pipeline(pipeline.pipeline());
                render_pass
                    .set_index_buffer(pipeline.index_buffer().slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_vertex_buffer(0, pipeline.vertex_buffer().slice(..));
                render_pass.set_vertex_buffer(1, pipeline.instance_buffer().slice(..));
                for (index, bind_group) in pipeline.bind_groups().iter().enumerate() {
                    render_pass.set_bind_group(index.try_into().unwrap(), Some(&**bind_group), &[]);
                }
                render_pass.draw_indexed(0..pipeline.num_indices(), 0, 0..pipeline.num_instances());
            }
        } // explicitly end lifetime of render pass before calling finish on the encoder

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
