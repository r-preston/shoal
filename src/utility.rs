use num_traits::Float;
use std::ops::{AddAssign, Deref, DerefMut};

pub type Position = cgmath::Point3<f32>;
pub type Velocity = cgmath::Vector3<f32>;
pub type Direction = cgmath::Vector3<f32>;

pub type Vec4 = cgmath::Vector4<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub type Radians = cgmath::Rad<f32>;
pub type Degrees = cgmath::Deg<f32>;

#[allow(non_snake_case)]
pub fn Degrees(deg: f32) -> Degrees {
    cgmath::Deg::<f32>(deg)
}

#[allow(non_snake_case)]
pub fn Radians(rad: f32) -> Radians {
    cgmath::Rad::<f32>(rad)
}
