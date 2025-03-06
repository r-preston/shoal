pub mod fish;
pub mod shark;

use crate::utility::{Mat3, Position, Radians, Vec3, Velocity};
use cgmath::{EuclideanSpace, InnerSpace, MetricSpace};

pub trait Actor {
    // return this Actor's position relative to the centre of the world
    fn position(&self) -> &Position;
    fn mutable_position(&mut self) -> &mut Position;

    // return the current velocity vector of this Actor
    fn velocity(&self) -> &Velocity;
    fn mutable_velocity(&mut self) -> &mut Velocity;

    fn turn_speed(&self) -> Radians;
    fn acceleration(&self) -> f32;
    fn speed_modifier(&self) -> f32;
    fn set_speed_modifier(&mut self, modifier: f32);

    fn update_position(&mut self, speed_modifier: f32) {
        let acceleration = (speed_modifier - self.speed_modifier()).signum() * self.acceleration();
        self.set_speed_modifier(self.speed_modifier() + acceleration);
        let v = *self.velocity() * self.speed_modifier();
        *self.mutable_position() += v;
    }

    fn move_in_direction(&mut self, direction: Velocity, speed_modifier: f32) {
        let mut axis = self.velocity().cross(direction).normalize();
        if axis.magnitude2() == 0.0 {
            axis = Vec3::new(0.0, 0.0, 1.0);
        }
        *self.mutable_velocity() =
            Mat3::from_axis_angle(axis, self.turn_speed() * speed_modifier) * self.velocity();
        self.update_position(speed_modifier);
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
