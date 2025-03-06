mod field;

use crate::actors::shark::Behaviour;
use crate::actors::{fish::Fish, shark::Shark, Actor};
use crate::utility::{Direction, Position, Vec3, Velocity};
use cgmath::{EuclideanSpace, InnerSpace, MetricSpace, Vector3};
use field::Field;
use std::cmp::Ordering;

pub struct World {
    radius: f32,
    fish: Vec<Fish>,
    sharks: Vec<Shark>,
    fish_density: Field<f32>,
    fish_direction: Field<Velocity>,
    cell_positions: Field<Position>,
    local_density_centre: Field<Position>,
    grid_average_offsets: Vec<i32>,
}

// squared distance from which fish will start running from threats
const FISH_FEAR_DISTANCE2: f32 = 64.0;
const MAX_FISH_DENSITY: f32 = 20.0;
const FISH_ALIGNMENT_WEIGHT: f32 = 0.1;
const FISH_CENTERING_WEIGHT: f32 = 1.0;
const ORIGIN_ATTRACTION_WEIGHT: f32 = 0.1;
const FISH_SIGHT_RANGE: f32 = 2.0;

impl World {
    pub fn new(radius: f32, grid_size: u32, fish_count: u32, shark_count: u32) -> World {
        let mut world = World {
            radius,
            fish: Vec::new(),
            sharks: Vec::new(),
            // grid with number of fish per cell
            fish_density: Field::new(radius, grid_size, 0.0),
            // grid with net direction of fish in each cell
            fish_direction: Field::new(radius, grid_size, Velocity::new(0.0, 0.0, 0.0)),
            // grid with world position of each cell
            cell_positions: Field::new(radius, grid_size, Position::new(0.0, 0.0, 0.0)),
            // grid with centre of mass of nearby fish for each cell
            local_density_centre: Field::new(radius, grid_size, Position::new(0.0, 0.0, 0.0)),
            grid_average_offsets: Vec::new(),
        };

        // generate actors
        world
            .fish
            .resize_with(fish_count as usize, || -> Fish { Fish::new(radius) });
        world.fish.shrink_to_fit();
        world
            .sharks
            .resize_with(shark_count as usize, || -> Shark { Shark::new(radius) });
        world.sharks.shrink_to_fit();

        // pre-calculate positions of grid cells
        for i in 0..world.cell_positions.cell_count() {
            let cell_pos = world.cell_positions.position_from_index(i as u32);
            world.cell_positions.add_to_field(i, cell_pos);
        }

        // calculate cell index offsets for averaging multiple cells within range
        let cell_width = radius * 2.0 / (grid_size as f32);
        let sight_range_cells: i32 = (FISH_SIGHT_RANGE / cell_width).floor() as i32;
        let sight_range_cells2 = sight_range_cells * sight_range_cells;
        for k in -sight_range_cells..=sight_range_cells {
            for j in -sight_range_cells..=sight_range_cells {
                for i in -sight_range_cells..=sight_range_cells {
                    if i * i + j * j + k * k <= sight_range_cells2 {
                        world
                            .grid_average_offsets
                            .push(i + (grid_size as i32) * (j + (grid_size as i32) * k));
                    }
                }
            }
        }

        return world;
    }

    pub fn update(&mut self, time: u32) {
        // process:
        // - update fields from fish
        // - update actors based on field values
        self.fish_density.clear();
        self.fish_direction.clear();
        for fish in self.fish.iter_mut() {
            let idx = self.fish_density.index_from_position(fish.position());
            fish.set_index(idx);
            self.fish_density.add_to_field(idx, 1.0);
            self.fish_direction
                .add_to_field(idx, fish.velocity().normalize());
        }
        // calculate nearest centre of mass of
        for (i, cell) in self.local_density_centre.data_enumerated() {
            let mut average_position = Position::new(0.0, 0.0, 0.0);
            let mut total_density: f32 = 0.0;
            for offset in self.grid_average_offsets.iter() {
                let index = ((i as i32) + offset) as usize;
                let density: f32 = *self.fish_density.field_value_checked(index).unwrap_or(&0.0);
                total_density += density;
                average_position += density
                    * self
                        .cell_positions
                        .field_value_checked(index)
                        .unwrap_or(&Position::new(0.0, 0.0, 0.0));
            }
            average_position /= total_density;
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
            let current_cell_index = fish.field_index();
            let current_cell_density = self.fish_density.field_value(current_cell_index);
            let current_cell_location = self.cell_positions.field_value(current_cell_index);
            let current_cell_direction = self.fish_direction.field_value(current_cell_index);

            // todo: be less shit
            let local_density_location = Position::new(0.0, 0.0, 0.0);

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
                fish.move_in_direction(
                    fish.position() - nearest_shark.unwrap().position(),
                    Fish::FLEE_SPEED_MODIFIER,
                );
            } else {
                // no nearby shark, do flocking logic
                // - align with other nearby fish
                // - move towards greatest local density of fish
                // - tendency to move towards centre of the world

                let tend_to_centre_weight =
                    FISH_CENTERING_WEIGHT * (1.0 - current_cell_density / MAX_FISH_DENSITY);

                let direction =
                    // align with others
                    FISH_ALIGNMENT_WEIGHT * current_cell_direction +
                    // move towards others
                    tend_to_centre_weight * (local_density_location - fish.position()) +
                    // central tendency
                    -1.0 * ORIGIN_ATTRACTION_WEIGHT * f32::exp(fish.position().magnitude2() / self.radius.powf(1.5)) * fish.position();

                fish.move_in_direction(direction, 1.0);
            }
        }

        // behaviour:
        // - evaluate current behaviour
        // - if hunting, pick the greatest density of fish and charge at it until past that point
        // - if roaming, patrol around the perimeter of the world
        for shark in self.sharks.iter_mut() {
            match shark.evaluate_behaviour() {
                Behaviour::Roaming(_) => {
                    if shark.position().magnitude2() < 0.8 * self.radius * self.radius {
                        shark.move_in_direction(
                            Direction::new(
                                shark.position().x,
                                shark.position().y,
                                shark.preferred_depth() - shark.position().z,
                            ),
                            1.0,
                        );
                    } else {
                        let perpendicular = shark.position().cross(Direction::new(0.0, 0.0, 1.0));
                        shark.move_in_direction(
                            Direction::new(perpendicular.x, perpendicular.y, 0.0),
                            1.0,
                        );
                    }
                }
                Behaviour::Hunting(origin) => {
                    if origin.dot(*shark.position())
                        < -0.5 * origin.magnitude() * shark.position().magnitude()
                    {
                        shark.end_hunt();
                    }
                    shark.move_in_direction(-1.0 * origin, Shark::HUNT_SPEED_MODIFIER);
                }
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
