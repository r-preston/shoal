mod field;

use std::cmp::Ordering;

use crate::actors::fish::Fish;
use crate::actors::shark::Shark;
use crate::actors::Actor;
use crate::utility::Velocity;
use cgmath::{InnerSpace, MetricSpace};
use field::Field;

pub struct World {
    radius: f32,
    fish: Vec<Fish>,
    sharks: Vec<Shark>,
    fish_density: Field<u32>,
    fish_direction: Field<Velocity>,
}

// squared distance from which fish will start running from threats
const FISH_FEAR_DISTANCE2: f32 = 10.0;
const MAX_FISH_DENSITY: u32 = 10;

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
        self.fish_density.clear();
        self.fish_direction.clear();
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
            // work out:
            // average fish direction in local cell
            // cell with greatest density of fish
            // nearest shark
            // current cell density
            let current_cell_index = self.fish_density.index_from_position(fish.position());
            let current_cell_density = self.fish_density.field_value_from_index(current_cell_index);
            let current_cell_location = self
                .fish_density
                .position_from_index(current_cell_index as u32);
            let current_cell_direction = self
                .fish_direction
                .field_value_from_index(current_cell_index);

            let densest_cell = self.fish_density.data().max_by(
                |cell_a: &(usize, &u32), cell_b: &(usize, &u32)| -> Ordering {
                    cell_a.1.cmp(cell_b.1)
                },
            );
            let densest_cell_location = self.fish_density.position_from_index(
                densest_cell
                    .unwrap_or_else(|| -> (usize, &u32) { (0, &0) })
                    .0 as u32,
            );

            let nearest_shark =
                self.sharks
                    .iter()
                    .min_by(|shark_a: &&Shark, shark_b: &&Shark| -> Ordering {
                        shark_a
                            .position()
                            .distance2(*fish.position())
                            .total_cmp(&shark_b.position().distance2(*fish.position()))
                    });

            if !nearest_shark.is_none_or(|shark: &Shark| -> bool {
                shark.position().distance2(*fish.position()) > FISH_FEAR_DISTANCE2
            }) {
                // shark nearby, run away from shark
                println!("RUN");
                fish.move_towards(fish.position() - nearest_shark.unwrap().position(), true);
            } else if current_cell_density > MAX_FISH_DENSITY {
                // current cell is too crowded, move away from centre of cell 
                println!("TOO CROWDED: {current_cell_density}");
                fish.move_towards(fish.position() - current_cell_location, false);
            } else {
                // no nearby shark, do flocking logic

                // align with others
                // if greatest density more than 1, move towards greatest density. otherwise, move towards (0,0)
                // avoid leaving the world area


            }
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
