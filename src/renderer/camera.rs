use crate::utility::*;
use cgmath::SquareMatrix;
use cgmath::{Angle, InnerSpace, MetricSpace};
use wgpu::util::DeviceExt;
use wgpu::BindGroupLayoutDescriptor;

pub struct CameraUniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub matrix: [[f32; 4]; 4],
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
            contents: bytemuck::cast_slice(&[0.0f32; 20]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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

    pub fn update(
        &self,
        queue: &wgpu::Queue,
        camera_matrix: &[[f32; 4]; 4],
        camera_position: &[f32; 3],
    ) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(camera_matrix));
        queue.write_buffer(
            &self.buffer,
            std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            bytemuck::cast_slice(camera_position),
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
        0.075
    }

    fn default_target() -> Position {
        Position::new(0.0, 0.0, 0.0)
    }

    pub fn set_projection(&mut self, matrix: Mat4) {
        self.projection = matrix;
    }

    pub fn position(&self) -> Position {
        let (sin_theta, cos_theta) = self.polar_angle.sin_cos();
        let (sin_phi, cos_phi) = self.azimuthal_angle.sin_cos();
        self.distance * Position::new(cos_theta * cos_phi, cos_theta * sin_phi, sin_theta)
    }

    pub fn view_matrix(&self) -> Mat4 {
        let position = self.position();
        let direction = Direction::from(position - self.target).normalize();
        let mut right = direction.cross(Camera::global_up());
        if right.magnitude2() == 0.0 {
            right = Direction::new(1.0, 0.0, 0.0);
        }
        let up = right.cross(direction);
        Mat4::look_at_rh(position, self.target, up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        OPENGL_TO_WGPU_MATRIX * self.projection
    }

    pub fn move_in_out(&mut self, amount: f32) {
        self.distance += amount * Camera::radial_sensitivity();
        self.distance = f32::max(1.0, self.distance);
    }

    pub fn move_up_down(&mut self, amount: f32) {
        let new_angle = self.polar_angle.0 + (amount * Camera::angular_sensitivity());
        self.polar_angle = Radians(
            new_angle
                .min(Radians::turn_div_4().0 - 0.05)
                .max(0.05 - Radians::turn_div_4().0),
        );
    }

    pub fn move_left_right(&mut self, amount: f32) {
        self.azimuthal_angle += Radians(amount * Camera::angular_sensitivity());
        self.azimuthal_angle = self.azimuthal_angle.normalize();
    }
}
