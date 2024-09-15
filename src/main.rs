fn main() {
    println!("Hello, world!");
}

// things I will need
fish - flocking, try and stay in sphere, avoid predators
sharks - roam near prey, sometimes attack
world - hold list of actors, passes data to renderer
renderer (main) - track timer and draw things
camera - moveable

// workflow:
have an array of fish positions
create a density map from fish positions each tick
update all fish with using this density map
update predators via density map
multiple density maps? fish vs threat