pub mod fish;
pub mod shark;

use crate::utility::{Position, Velocity};
use cgmath::{EuclideanSpace, InnerSpace, MetricSpace};

pub trait Actor {
    // return this Actor's position relative to the centre of the world
    fn position(&self) -> &Position;
    fn mutable_position(&mut self) -> &mut Position;

    // return the current velocity vector of this Actor
    fn velocity(&self) -> &Velocity;
    fn mutable_velocity(&mut self) -> &mut Velocity;

    fn update_position(&mut self) {
        *self.mutable_position() = Position::from_vec(self.position().to_vec() + self.velocity());
    }

    // return distance of the Actor from the centre of the world
    fn distance(&self) -> f32 {
        self.position().distance(Position::new(0.0, 0.0, 0.0))
    }

    // return distance squared of the actor from the centre of the world
    fn distance2(&self) -> f32 {
        self.position().distance2(Position::new(0.0, 0.0, 0.0))
    }

    // return magnitude of Actor's velocity
    fn speed(&self) -> f32 {
        self.velocity().magnitude()
    }

    // return magnitude of Actor's velocity squared
    fn speed2(&self) -> f32 {
        self.velocity().magnitude2()
    }
}
