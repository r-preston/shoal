mod field;

use std::cmp::Ordering;

use crate::actors::fish::Fish;
use crate::actors::shark::Shark;
use crate::actors::Actor;
use crate::utility::Velocity;
use cgmath::InnerSpace;
use field::Field;

pub struct World {
    radius: f32,
    fish: Vec<Fish>,
    sharks: Vec<Shark>,
    fish_density: Field<u32>,
    fish_direction: Field<Velocity>,
}

impl World {
    pub fn new(radius: f32, grid_size: u32, fish_count: u32, shark_count: u32) -> World {
        let mut world = World {
            radius,
            fish: Vec::new(),
            sharks: Vec::new(),
            fish_density: Field::new(radius, grid_size, 0),
            fish_direction: Field::new(radius, grid_size, Velocity::new(0.0, 0.0, 0.0)),
        };

        for _n in 0..fish_count {
            world
                .fish
                .resize_with(fish_count as usize, || -> Fish { Fish::new(radius) });
        }
        for _n in 0..shark_count {
            world.sharks.resize_with(shark_count as usize, Shark::new);
        }

        world.fish.shrink_to_fit();
        world.sharks.shrink_to_fit();
        return world;
    }

    pub fn update(&mut self, time: u32) {
        // process:
        // - update fields from actors
        // - update actors based on field values
        
        for fish in self.fish.iter() {
            self.fish_density.add_to_field(fish.position(), 1);
            self.fish_direction
                .add_to_field(fish.position(), fish.velocity().normalize());
        }
        // fish behaviour:
        // - avoid predators
        // - don't go outside world boundary
        // - move towards greatest local density of other fish
        // - align with local fish direction
        // - don't overcrowd current cell
        for fish in self.fish.iter_mut() {
            let avg_direction = self.fish_direction.field_value(fish.position());
            let densest_cell = self.fish_density.data().max_by(
                |cell_a: &(usize, &u32), cell_b: &(usize, &u32)| -> Ordering {
                    // !TODO: filter out cells too far away
                    cell_a.cmp(cell_b)
                },
            );
            for cell in self.fish_density.data() {}
            // work out:
            // figure out local direction
            // figure out centre of local density
            // nearest shark
            // current cell density

            //fish.update_velocity(self.fish_density);
            //fish.move();
        }
        
    }

    pub fn fish(&self) -> &Vec<Fish> {
        &self.fish
    }

    pub fn sharks(&self) -> &Vec<Shark> {
        &self.sharks
    }

    pub fn size(&self) -> f32 {
        self.radius
    }
}
