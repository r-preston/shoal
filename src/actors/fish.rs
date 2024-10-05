use crate::actors::Actor;
use crate::Position;
use crate::Velocity;

pub struct Fish {
    m_position: Position,
    m_velocity: Velocity,
}

impl Fish {
    pub fn new() -> Fish {
        Fish {
            m_position: Position(0.0, 0.0, 0.0),
            m_velocity: Velocity(0.0, 0.0, 0.0),
        }
    }
}

impl Actor for Fish {
    fn position(&self) -> &Position {
        return &self.m_position;
    }
    fn velocity(&self) -> &Velocity {
        return &self.m_velocity;
    }
}
