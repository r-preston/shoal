use crate::actors::Actor;
use crate::utility::Position;
use crate::utility::Velocity;

pub struct Shark {
    m_position: Position,
    m_velocity: Velocity,
}

impl Shark {
    pub fn new() -> Shark {
        Shark {
            m_position: Position::new(0.0, 0.0, 0.0),
            m_velocity: Velocity::new(0.0, 0.0, 0.0),
        }
    }
}

impl Actor for Shark {
    fn position(&self) -> &Position {
        return &self.m_position;
    }
    fn velocity(&self) -> &Velocity {
        return &self.m_velocity;
    }
}
