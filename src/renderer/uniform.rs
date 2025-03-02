use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use wgpu::BindGroupLayoutDescriptor;

pub struct UniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
}

impl UniformBuffer {
    pub fn new(device: &wgpu::Device) -> UniformBuffer {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
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
            label: Some("uniform_bind_group_layout"),
        });
        UniformBuffer {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("uniform_bind_group"),
            }),
            buffer: buffer,
            layout: layout,
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
