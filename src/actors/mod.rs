pub mod fish;
pub mod shark;

pub trait Actor {
    // return this Actor's position relative to the centre of the world
    fn position(&self) -> &crate::Position;

    // return the current velocity vector of this Actor
    fn velocity(&self) -> &crate::Velocity;

    // return distance of the Actor from the centre of the world
    fn distance(&self) -> crate::PositionType {
        self.distance2().sqrt()
    }

    // return distance squared of the actor from the centre of the world
    fn distance2(&self) -> crate::PositionType {
        let p = self.position();
        p.0 * p.0 + p.1 * p.1 + p.2 * p.2
    }

    // return magnitude of Actor's velocity
    fn speed(&self) -> crate::VelocityType {
        self.distance2().sqrt()
    }

    // return magnitude of Actor's velocity squared
    fn speed2(&self) -> crate::VelocityType {
        let v = self.velocity();
        v.0 * v.0 + v.1 * v.1 + v.2 * v.2
    }
}
