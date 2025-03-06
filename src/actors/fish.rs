use crate::actors::Actor;
use crate::utility::{Position, Radians, Vec3, Velocity};
use cgmath::InnerSpace;

pub struct Fish {
    field_index: usize,
    turn_speed: Radians,
    position: Position,
    velocity: Velocity,
    direction_tweak: cgmath::Quaternion<f32>,
}

impl Fish {
    pub const FLEE_SPEED_MODIFIER: f32 = 2.0;
    const DEFAULT_SPEED: f32 = 0.1;
    const DEFAULT_TURN_SPEED_RADIANS: f32 = 0.025;
    const ACCELERATION: f32 = 0.01;

    pub fn new(world_size: f32) -> Fish {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut random = |min: f32, max: f32| -> f32 { rng.random::<f32>() * (max - min) + min };

        let distance_from_origin: f32 = world_size * random(0.0, 1.0);

        let speed = Self::DEFAULT_SPEED * random(0.6, 1.5);

        let position = Position::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0))
            .normalize_to(distance_from_origin);

        let velocity = Velocity::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0))
            .normalize_to(speed);

        let direction_tweak = cgmath::Quaternion::from_sv(
            1.0,
            Vec3::new(random(0.0, 0.1), random(0.0, 0.1), random(0.0, 0.1)),
        );

        Fish {
            field_index: 0,
            turn_speed: Radians(Self::DEFAULT_TURN_SPEED_RADIANS * random(0.6, 1.5)),
            position,
            velocity,
            direction_tweak,
        }
    }

    pub fn field_index(&self) -> usize {
        self.field_index
    }

    pub fn set_index(&mut self, idx: usize) {
        self.field_index = idx;
    }

    pub fn direction_matrix(&self) -> cgmath::Quaternion<f32> {
        cgmath::Quaternion::from_arc(
            Vec3::new(1.0, 0.0, 0.0),
            self.velocity,
            Some(Vec3::new(1.0, 0.0, 0.0)),
        ) * self.direction_tweak
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
    fn turn_speed(&self) -> Radians {
        self.turn_speed
    }
    fn acceleration(&self) -> f32 {
        Self::ACCELERATION
    }
    fn speed_modifier(&self) -> f32 {
        1.0
    }
    fn set_speed_modifier(&mut self, _modifier: f32) {
        ()
    }
}
