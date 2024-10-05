mod actors;
mod camera;
mod world;

type VelocityType = f32;
type PositionType = f32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Velocity(VelocityType, VelocityType, VelocityType);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Position(PositionType, PositionType, PositionType);

impl std::ops::AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Velocity) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

fn main() {
    let _world = world::World::new(100.0, 5, 10, 0);
}

/*
// things I will need
fish - flocking, try and stay in sphere, avoid predators. need velocity, position
sharks - roam near prey, sometimes attack
world - hold list of actors, passes data to renderer
renderer (main) - track timer and draw things
camera - moveable

// workflow:
have an array of fish positions and velocities
create a density and velocity field map from fish positions each tick
update all fish with using these maps
update predators using maps
multiple density maps? fish vs threat
*/
