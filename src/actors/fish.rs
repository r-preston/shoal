use crate::actors::Actor;
use crate::utility::Direction;
use crate::utility::Mat3;
use crate::utility::Position;
use crate::utility::Radians;
use crate::utility::Vec3;
use crate::utility::Velocity;
use crate::world::World;
use cgmath::InnerSpace;
use cgmath::Matrix3;
use cgmath::MetricSpace;
use cgmath::Rotation3;
use cgmath::Vector3;
use rand::prelude::*;

pub struct Fish {
    field_index: usize,
    speed: f32,
    turn_speed: Radians,
    position: Position,
    velocity: Velocity,
    direction_tweak: cgmath::Quaternion<f32>,
}

const DEFAULT_SPEED: f32 = 0.1;
const FLEE_SPEED_MODIFIER: f32 = 2.0;
const DEFAULT_TURN_SPEED_RADIANS: f32 = 0.025;

impl Fish {
    pub fn new(world_size: f32) -> Fish {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut random = |min: f32, max: f32| -> f32 { rng.random::<f32>() * (max - min) + min };

        let distance_from_origin: f32 = world_size * random(0.0, 1.0);

        let speed = DEFAULT_SPEED * random(0.6, 1.5);

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
            speed,
            turn_speed: Radians(DEFAULT_TURN_SPEED_RADIANS * random(0.6, 1.5)),
            position,
            velocity,
            direction_tweak,
        }
    }

    // !todo: implement a max turning speed
    pub fn move_towards(&mut self, direction: Velocity, is_afraid: bool) {
        let mut axis = self.velocity.cross(direction).normalize();
        if axis.magnitude2() == 0.0 {
            axis = Vector3::new(0.0, 0.0, 1.0);
        }
        self.velocity = Mat3::from_axis_angle(axis, self.turn_speed) * self.velocity;
        self.update_position();
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
}
