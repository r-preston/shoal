use crate::utility::*;
use crate::world::World;
use cgmath::SquareMatrix;
use cgmath::{Angle, InnerSpace, MetricSpace};
use wgpu::hal::auxil::MAX_I32_BINDING_SIZE;

struct Perspective {
    fovy: Radians,
    aspect: f32,
    near: f32,
    far: f32,
    matrix: Mat4,
}

impl Perspective {
    pub fn new(fovy: Radians, aspect: f32, near: f32, far: f32) -> Perspective {
        let mut perspective = Perspective {
            fovy,
            aspect,
            near,
            far,
            matrix: Mat4::identity(),
        };
        perspective.calculate_matrix();
        return perspective;
    }

    fn calculate_matrix(&mut self) {
        self.matrix = Mat4::from(cgmath::PerspectiveFov::<f32> {
            fovy: self.fovy,
            aspect: self.aspect,
            near: self.near,
            far: self.far,
        });
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.calculate_matrix();
    }
}

pub struct Camera {
    target: Position,
    distance: f32,
    min_distance: f32,
    max_distance: f32,
    azimuthal_angle: Radians,
    polar_angle: Radians,
    perspective: Perspective,
}

impl Camera {
    const RADIAL_SENSITIVITY: f32 = 1.0;
    const ANGULAR_SENSITIVITY: f32 = 0.075;
    const DEFAULT_DISTANCE_MULTIPLIER: f32 = 1.5;
    const MIN_DISTANCE: f32 = 1.0;
    const MAX_DISTANCE_MULTIPLIER: f32 = 4.0;

    pub fn new(world: &World, config: &wgpu::SurfaceConfiguration) -> Camera {
        let distance = world.size() * Self::DEFAULT_DISTANCE_MULTIPLIER;
        let max_distance = world.size() * Self::MAX_DISTANCE_MULTIPLIER;
        let min_distance = Self::MIN_DISTANCE;
        Self {
            target: Camera::default_target(),
            distance,
            min_distance,
            max_distance,
            azimuthal_angle: Radians(0.0),
            polar_angle: Radians(0.0),
            perspective: Perspective::new(
                Radians::from(Degrees(90.0)),
                (config.width as f32) / (config.height as f32),
                min_distance,
                max_distance + world.size(),
            ),
        }
    }

    fn global_up() -> Direction {
        Direction::new(0.0, 0.0, 1.0)
    }

    fn default_target() -> Position {
        Position::new(0.0, 0.0, 0.0)
    }

    pub fn set_aspect(&mut self, ratio: f32) {
        self.perspective.set_aspect(ratio);
    }

    pub fn position(&self) -> Position {
        let (sin_theta, cos_theta) = self.polar_angle.sin_cos();
        let (sin_phi, cos_phi) = self.azimuthal_angle.sin_cos();
        self.distance * Position::new(cos_theta * cos_phi, cos_theta * sin_phi, sin_theta)
    }

    pub fn view_matrix(&self) -> Mat4 {
        let position = self.position();
        let direction = Direction::from(position - self.target).normalize();
        let mut right = direction.cross(Camera::global_up());
        if right.magnitude2() == 0.0 {
            right = Direction::new(1.0, 0.0, 0.0);
        }
        let up = right.cross(direction);
        Mat4::look_at_rh(position, self.target, up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        OPENGL_TO_WGPU_MATRIX * self.perspective.matrix
    }

    pub fn move_in_out(&mut self, amount: f32) {
        self.distance += amount * Self::RADIAL_SENSITIVITY;
        self.distance = f32::max(self.min_distance, self.distance);
        self.distance = f32::min(self.max_distance, self.distance);
    }

    pub fn move_up_down(&mut self, amount: f32) {
        let new_angle = self.polar_angle.0 + (amount * Self::ANGULAR_SENSITIVITY);
        self.polar_angle = Radians(
            new_angle
                .min(Radians::turn_div_4().0 - 0.05)
                .max(0.05 - Radians::turn_div_4().0),
        );
    }

    pub fn move_left_right(&mut self, amount: f32) {
        self.azimuthal_angle += Radians(amount * Self::ANGULAR_SENSITIVITY);
        self.azimuthal_angle = self.azimuthal_angle.normalize();
    }
}
