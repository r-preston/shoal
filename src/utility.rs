use num_traits::Float;
use std::ops::{Deref, AddAssign, DerefMut};
use cgmath;

pub type PositionType = f32;
pub type VelocityType = f32;
pub type DirectionType = f32;
pub type Position = cgmath::Vector3<PositionType>; 
pub type Velocity = cgmath::Vector3<VelocityType>;
pub type Direction = cgmath::Vector3<DirectionType>;