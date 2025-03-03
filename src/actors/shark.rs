use crate::actors::Actor;
use crate::utility::Position;
use crate::utility::Velocity;

const CRUISE_SPEED: f32 = 0.05;
const HUNT_SPEED: f32 = 0.1;

pub struct Shark {
    position: Position,
    velocity: Velocity,
}

impl Shark {
    pub fn new() -> Shark {
        Shark {
            position: Position::new(0.0, 0.0, 0.0),
            velocity: Velocity::new(0.0, 0.0, 0.0),
        }
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
}
