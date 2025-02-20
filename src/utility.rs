use cgmath::EuclideanSpace;
use cgmath::{Point3, Vector3, Vector4};
use num_traits::Float;
use std::ops::{AddAssign, Deref, DerefMut};

pub type Position = Point3<f32>;
pub type Velocity = Vector3<f32>;
pub type Direction = Vector3<f32>;

pub type Vec4 = Vector4<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub type Radians = cgmath::Rad<f32>;
pub type Degrees = cgmath::Deg<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub struct Instance {
    pub position: Position,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

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
            model: (cgmath::Matrix4::from_translation(self.position.to_vec())
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}
