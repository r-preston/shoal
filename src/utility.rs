use num_traits::Float;
use std::ops::{Deref, AddAssign, DerefMut};

pub type PositionType = f32;
pub type VelocityType = f32; 

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector<T>(T, T, T);

impl<T: Float> Vector<T> {

    pub fn x(&self) -> T {
        self.0
    }
    pub fn y(&self) -> T {
        self.1
    }
    pub fn z(&self) -> T {
        self.2
    }

    pub fn magnitude2(&self) -> T {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z()) 
    }

    pub fn magnitude(&self) -> T {
        self.magnitude2().sqrt()
    }

}

impl<T: Float + AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Position(pub Vector<PositionType>);

impl Position {
    pub fn new(x: PositionType, y: PositionType, z: PositionType) -> Position {
        Position(Vector(x, y, z))
    }
}

impl Deref for Position {
    type Target = Vector<PositionType>;
    fn deref(&self) -> &Vector<PositionType> {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Vector<PositionType> {
        &mut self.0
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.0 += other.0;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Velocity(pub Vector<VelocityType>);

impl Velocity {
    pub fn new(x: VelocityType, y: VelocityType, z: VelocityType) -> Velocity {
        Velocity(Vector(x, y, z))
    }
}

impl Deref for Velocity {
    type Target = Vector<VelocityType>;
    fn deref(&self) -> &Vector<VelocityType> {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Vector<VelocityType> {
        &mut self.0
    }
} 

impl AddAssign for Velocity {
    fn add_assign(&mut self, other: Velocity) {
        self.0 += other.0;
    }
}