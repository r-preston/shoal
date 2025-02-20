use cgmath::EuclideanSpace;
use num_traits::Float;
use std::ops::{AddAssign, Deref, DerefMut};

pub type Position = cgmath::Point3<f32>;
pub type Velocity = cgmath::Vector3<f32>;
pub type Direction = cgmath::Vector3<f32>;

pub type Vec4 = cgmath::Vector4<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub type Radians = cgmath::Rad<f32>;
pub type Degrees = cgmath::Deg<f32>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Position,
}

pub struct Instance {
    pub position: Position,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: Position::new(x, y, z),
        }
    }
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
