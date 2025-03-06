use std::thread::Thread;

use crate::actors::Actor;
use crate::utility::{Direction, Mat3, Position, Radians, Vec3, Velocity};
use crate::world;
use cgmath::{InnerSpace, Matrix3, MetricSpace, Rotation3, Vector3};
use rand::rngs::ThreadRng;

#[derive(Copy, Clone)]
pub enum Behaviour {
    Roaming(u32),      // roaming for a number of updates
    Hunting(Position), // hunting from origin point
}

pub struct Shark {
    position: Position,
    velocity: Velocity,
    base_speed: f32,
    speed_modifier: f32,
    behaviour: Behaviour, // current AI shark will follow
    roam_timer: u32,      // number of updates the shark has been using this AI
    preferred_depth: f32, // depth that shark will patrol at
    rng: ThreadRng,
}

impl Shark {
    pub const HUNT_SPEED_MODIFIER: f32 = 10.0;
    const TURN_SPEED_RADIANS: f32 = 0.01;
    const DEFAULT_SPEED: f32 = 0.05;
    const ACCELERATION: f32 = 0.05;

    pub fn new(world_size: f32) -> Shark {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut random = |min: f32, max: f32| -> f32 { rng.random::<f32>() * (max - min) + min };

        let distance_from_origin: f32 = world_size * random(0.0, 1.0);
        let speed = Self::DEFAULT_SPEED * random(0.6, 1.5);

        let position = Position::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0))
            .normalize_to(distance_from_origin);

        let velocity = Velocity::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0))
            .normalize_to(speed);

        Shark {
            position,
            velocity,
            base_speed: speed,
            speed_modifier: 1.0,
            behaviour: Behaviour::Roaming(60 * random(5.0, 20.0) as u32),
            roam_timer: 0,
            preferred_depth: world_size * random(-0.3, 0.3),
            rng,
        }
    }

    pub fn preferred_depth(&self) -> f32 {
        self.preferred_depth
    }

    pub fn evaluate_behaviour(&mut self) -> Behaviour {
        match self.behaviour {
            Behaviour::Roaming(time) => {
                if self.roam_timer > time {
                    self.behaviour = Behaviour::Hunting(self.position);
                } else {
                    self.roam_timer += 1;
                }
            }
            _ => {}
        }
        self.behaviour
    }

    pub fn end_hunt(&mut self) {
        use rand::Rng;
        let mut random =
            |min: f32, max: f32| -> f32 { self.rng.random::<f32>() * (max - min) + min };
        self.behaviour = Behaviour::Roaming(60 * random(10.0, 30.0) as u32);
        self.roam_timer = 0;
    }
}

impl Actor for Shark {
    fn position(&self) -> &Position {
        &self.position
    }
    fn mutable_position(&mut self) -> &mut Position {
        &mut self.position
    }
    fn velocity(&self) -> &Velocity {
        &self.velocity
    }
    fn mutable_velocity(&mut self) -> &mut Velocity {
        &mut self.velocity
    }
    fn turn_speed(&self) -> crate::utility::Radians {
        Radians(Self::TURN_SPEED_RADIANS)
    }
    fn acceleration(&self) -> f32 {
        Self::ACCELERATION
    }
    fn speed_modifier(&self) -> f32 {
        self.speed_modifier
    }
    fn set_speed_modifier(&mut self, modifier: f32) {
        self.speed_modifier = modifier
    }
}
