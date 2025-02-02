use crate::utility::*;
use cgmath::{Angle, InnerSpace, MetricSpace};

pub struct Camera {
    target: Position,
    distance: f32,
    azimuthal_angle: Radians,
    polar_angle: Radians,
    perspective_matrix: Mat4,
}

impl Camera {
    fn global_up() -> Direction {
        Direction::new(0.0, 0.0, 1.0)
    }

    fn radial_sensitivity() -> f32 {
        1.0
    }

    fn angular_sensitivity() -> f32 {
        1.0
    }

    fn default_target() -> Position {
        Position::new(0.0, 0.0, 0.0)
    }

    pub fn position(&self) -> Position {
        let (sin_theta, cos_theta) = self.polar_angle.sin_cos();
        let (sin_phi, cos_phi) = self.azimuthal_angle.sin_cos();
        self.distance * Position::new(cos_theta * cos_phi, cos_theta * sin_phi, sin_theta)
    }

    fn generate_view_matrix(&self) -> Mat4 {
        let position = self.position();
        let direction = Direction::from(self.position() - self.target).normalize();
        let right = Camera::global_up().cross(direction);
        let up = direction.cross(right);
        return Mat4::look_at_rh(position, self.target, up);
    }

    pub fn new(distance: f32) -> Camera {
        let perspective = cgmath::PerspectiveFov::<f32> {
            fovy: Radians::from(Degrees(90.0)),
            aspect: 1.0,
            near: 0.0,
            far: 100.0,
        };
        Self {
            target: Camera::default_target(),
            distance,
            azimuthal_angle: Radians(0.0),
            polar_angle: Radians(0.0),
            perspective_matrix: Mat4::from(perspective),
        }
    }

    pub fn move_in_out(&mut self, amount: f32) {
        self.distance += (amount * Camera::radial_sensitivity());
        self.distance = f32::max(1.0, self.distance);
    }

    pub fn move_up_down(&mut self, amount: f32) {
        let new_angle = self.polar_angle.0 + (amount * Camera::angular_sensitivity());
        self.polar_angle = Radians(
            new_angle
                .max(Radians::turn_div_4().0 - 0.05)
                .min(0.05 - Radians::turn_div_4().0),
        );
    }

    pub fn move_left_right(&mut self, amount: f32) {
        self.azimuthal_angle += Radians(amount * Camera::angular_sensitivity());
        self.azimuthal_angle = self.azimuthal_angle.normalize();
    }

    pub fn vp_matrix(&self) -> Mat4 {
        self.generate_view_matrix() * self.perspective_matrix
    }
}
