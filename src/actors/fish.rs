use crate::actors::Actor;
use crate::utility::Direction;
use crate::utility::Position;
use crate::utility::Vec3;
use crate::utility::Velocity;
use crate::world::World;
use cgmath::InnerSpace;
use rand::prelude::*;

pub struct Fish {
    position: Position,
    velocity: Velocity,
}

const DEFAULT_SPEED: f32 = 0.05;
const FLEE_SPEED: f32 = 0.1;

impl Fish {
    pub fn new(world_size: f32) -> Fish {
        use rand::Rng;

        let mut rng = rand::rng();
        let mut random: [f32; 6] = rng.random();
        let random_scale: f32 = world_size * rng.random::<f32>();
        for r in random.iter_mut() {
            *r = 2.0 * (*r) - 1.0;
        }
        let position = Vec3::new(random[0], random[1], random[2]).normalize_to(random_scale);
        Fish {
            position: Position::new(position.x, position.y, position.z),
            velocity: Velocity::new(random[3], random[4], random[5])
                .normalize_to(DEFAULT_SPEED),
        }
    }

    // !todo: implement a max turning speed
    pub fn move_towards(&mut self, direction: Velocity, is_afraid: bool) {
        self.velocity = direction.normalize_to(if is_afraid {FLEE_SPEED} else {DEFAULT_SPEED});
        self.update_position();
    }
}

impl Actor for Fish {
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
