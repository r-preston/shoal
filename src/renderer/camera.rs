use crate::utility::*;
use cgmath::SquareMatrix;
use cgmath::{Angle, InnerSpace, MetricSpace};
use wgpu::util::DeviceExt;
use wgpu::BindGroupLayoutDescriptor;

pub struct CameraUniformBuffer {
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    matrix: [[f32; 4]; 4],
}

pub struct Camera {
    target: Position,
    distance: f32,
    azimuthal_angle: Radians,
    polar_angle: Radians,
    projection: Mat4,
}

impl CameraUniformBuffer {
    pub fn new(device: &wgpu::Device) -> CameraUniformBuffer {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[0.0; 16]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
        CameraUniformBuffer {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            }),
            buffer: buffer,
            layout: layout,
            matrix: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, camera: &Camera) {
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[camera.view_projection_matrix()]),
        );
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
    
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

impl Camera {
    pub fn new(distance: f32) -> Camera {
        Self {
            target: Camera::default_target(),
            distance,
            azimuthal_angle: Radians(0.0),
            polar_angle: Radians(0.0),
            projection: cgmath::Matrix4::identity(),
        }
    }

    fn global_up() -> Direction {
        Direction::new(0.0, 0.0, 1.0)
    }

    fn radial_sensitivity() -> f32 {
        1.0
    }

    fn angular_sensitivity() -> f32 {
        1.0
    }

    fn default_target() -> Position {
        Position::new(0.0, 0.0, 0.0)
    }

    pub fn position(&self) -> Position {
        let (sin_theta, cos_theta) = self.polar_angle.sin_cos();
        let (sin_phi, cos_phi) = self.azimuthal_angle.sin_cos();
        self.distance * Position::new(cos_theta * cos_phi, cos_theta * sin_phi, sin_theta)
    }

    pub fn view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let position = self.position();
        let direction = Direction::from(self.position() - self.target).normalize();
        let right = Camera::global_up().cross(direction);
        let up = direction.cross(right);

        return (OPENGL_TO_WGPU_MATRIX
            * self.projection
            * Mat4::look_at_rh(position, self.target, up))
        .into();
    }

    pub fn move_in_out(&mut self, amount: f32) {
        self.distance += amount * Camera::radial_sensitivity();
        self.distance = f32::max(1.0, self.distance);
    }

    pub fn move_up_down(&mut self, amount: f32) {
        let new_angle = self.polar_angle.0 + (amount * Camera::angular_sensitivity());
        self.polar_angle = Radians(
            new_angle
                .max(Radians::turn_div_4().0 - 0.05)
                .min(0.05 - Radians::turn_div_4().0),
        );
    }

    pub fn move_left_right(&mut self, amount: f32) {
        self.azimuthal_angle += Radians(amount * Camera::angular_sensitivity());
        self.azimuthal_angle = self.azimuthal_angle.normalize();
    }
}
