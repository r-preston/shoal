pub mod field;

use crate::actors::fish::Fish;
use crate::actors::shark::Shark;
use crate::utility::Velocity;
use field::Field;

pub struct World {
    radius: f32,
    fish: Vec<Fish>,
    sharks: Vec<Shark>,
    fish_density: Field<u32>,
    shark_density: Field<u32>,
    fish_direction: Field<Velocity>,
}

impl World {
    pub fn new(radius: f32, grid_size: u32, fish_count: usize, shark_count: usize) -> World {
        let mut world = World {
            radius,
            fish: Vec::new(),
            sharks: Vec::new(),
            fish_density: Field::new(radius, grid_size, 0),
            shark_density: Field::new(radius, grid_size, 0),
            fish_direction: Field::new(radius, grid_size, Velocity::new(0.0, 0.0, 0.0)),
        };

        for _n in 0..fish_count {
            world.fish.resize_with(fish_count, Fish::new);
        }
        for _n in 0..shark_count {
            world.sharks.resize_with(shark_count, Shark::new);
        }

        world.fish.shrink_to_fit();
        world.sharks.shrink_to_fit();
        return world;
    }

    pub fn update(&mut self) {
        ()
    }

    pub fn size(&self) -> f32 {
        self.radius
    }
}
