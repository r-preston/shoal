use cgmath::Vector3;

pub type Position = Vector3<f32>;
pub type Velocity = Vector3<f32>;
pub type Direction = Vector3<f32>;

pub type Vec3 = Vector3<f32>;
pub type Mat3 = cgmath::Matrix3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub type Radians = cgmath::Rad<f32>;
pub type Degrees = cgmath::Deg<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub struct Instance {
    pub scale: f32,
    pub position: Position,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

#[rustfmt::skip]
// WebGPU uses DirectX's moronic normalised coordinate system where z goes from 0 to 1.
// This crate works in OGL coordinates as nature intended (-1 to 1)
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    // note: this representation is the transpose of the actual matrix
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

#[allow(non_snake_case)]
pub fn Degrees(deg: f32) -> Degrees {
    cgmath::Deg::<f32>(deg)
}

#[allow(non_snake_case)]
pub fn Radians(rad: f32) -> Radians {
    cgmath::Rad::<f32>(rad)
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation)
                * cgmath::Matrix4::from_scale(self.scale))
            .into(),
        }
    }
}
