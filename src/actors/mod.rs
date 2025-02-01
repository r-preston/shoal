pub mod fish;
pub mod shark;

use crate::utility::Position;
use crate::utility::Velocity;
use crate::utility::PositionType;
use crate::utility::VelocityType;
use cgmath::InnerSpace;

pub trait Actor {
    // return this Actor's position relative to the centre of the world
    fn position(&self) -> &Position;

    // return the current velocity vector of this Actor
    fn velocity(&self) -> &Velocity;

    // return distance of the Actor from the centre of the world
    fn distance(&self) -> PositionType {
        self.position().magnitude()
    }

    // return distance squared of the actor from the centre of the world
    fn distance2(&self) -> PositionType {
        self.position().magnitude2()
    }

    // return magnitude of Actor's velocity
    fn speed(&self) -> VelocityType {
        self.velocity().magnitude()
    }

    // return magnitude of Actor's velocity squared
    fn speed2(&self) -> VelocityType {
        self.velocity().magnitude2()
    }
}
