use crate::actors::Actor;
use crate::utility::Position;
use crate::utility::Vec3;
use crate::utility::Velocity;
use crate::world::World;
use cgmath::InnerSpace;
use rand::prelude::*;

pub struct Fish {
    m_position: Position,
    m_velocity: Velocity,
}

const DEFAULT_VELOCITY: f32 = 0.05;
const MAX_VELOCITY: f32 = 0.1;

impl Fish {
    pub fn new(world_size: f32) -> Fish {
        use rand::Rng;

        let mut rng = rand::rng();
        let random: [f32; 6] = rng.random();
        let position = Vec3::new(random[0], random[1], random[2]).normalize_to(world_size);
        Fish {
            m_position: Position::new(position.x, position.y, position.z),
            m_velocity: Velocity::new(random[3], random[4], random[5])
                .normalize_to(DEFAULT_VELOCITY),
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
