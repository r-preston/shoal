use crate::utility::{Position, Velocity};

pub struct Camera {
    position: Position,
    direction: Velocity
}

impl Camera {
    pub fn new(position: Position) -> Camera {
        let direction = Velocity::new(-position.x().signum(), 0.0, 0.0);
        Self {
            position,
            direction
        }
    }
}