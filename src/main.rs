mod actors;
mod camera;
mod world;
mod utility;
mod renderer;


fn main() {
    let _world = world::World::new(100.0, 5, 10, 0);
    renderer::run();    
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
