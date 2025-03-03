use core::slice;
use std::{cell, iter::Enumerate};

use num_traits::Euclid;

use crate::utility::Position;

pub struct Field<FieldType> {
    data: Vec<FieldType>,
    default_value: FieldType,
    cells_per_row: u32,
    world_size: f32,     // radius of sphere, equals half of total grid width
    position_shift: f32, // = cells_per_row / 2
    position_scale: f32, // = cells_per_row / (2 * world_size)
    index_scale: f32,    // = world_size / cells_per_row
    max_row_index: f32,  // cells_per_row - 1
}

impl<FieldType: Copy + std::ops::AddAssign> Field<FieldType> {
    pub fn new(world_size: f32, cells_per_row: u32, default_value: FieldType) -> Field<FieldType> {
        if cells_per_row % 2 == 0 {
            panic!("Attempted to create field with even number of cells per row");
        }
        let half_cells_per_row = (cells_per_row as f32) / 2.0;
        let mut field = Field {
            data: Vec::new(),
            default_value,
            cells_per_row,
            world_size,
            position_shift: half_cells_per_row,
            position_scale: half_cells_per_row / world_size,
            index_scale: world_size / (cells_per_row as f32),
            max_row_index: (cells_per_row - 1) as f32,
        };
        let vec_size = usize::try_from(cells_per_row.pow(3)).unwrap();
        field.data.resize(vec_size, default_value);
        field.data.shrink_to_fit();
        return field;
    }

    pub fn clear(&mut self) {
        self.data.fill(self.default_value);
    }

    pub fn field_value(&self, pos: &Position) -> FieldType {
        self.field_value_from_index(self.index_from_position(pos))
    }

    pub fn field_value_from_index(&self, index: usize) -> FieldType {
        self.data[index]
    }

    pub fn set_field(&mut self, value: FieldType) {
        self.data.fill(value);
    }

    pub fn add_to_field(&mut self, pos: &Position, value: FieldType) {
        let index = self.index_from_position(pos);
        self.data[index] += value;
    }

    pub fn data(&self) -> Enumerate<std::slice::Iter<FieldType>> {
        self.data.iter().enumerate()
    }

    pub fn position_from_index(&self, index: u32) -> Position {
        let slice_size = self.cells_per_row * self.cells_per_row;
        let (z_index, z_remainder) = index.div_rem_euclid(&slice_size);
        let (y_index, x_index) = z_remainder.div_rem_euclid(&self.cells_per_row);

        let scale_index =
            |i: u32| -> f32 { self.index_scale * ((2 * i + 1) as f32) - self.world_size };
        Position::new(
            scale_index(x_index),
            scale_index(y_index),
            scale_index(z_index),
        )
    }

    pub fn index_from_position(&self, pos: &Position) -> usize {
        let scale_component = |x: f32| -> u32 {
            (pos.x * self.position_scale + self.position_shift)
                .floor()
                .min(self.max_row_index)
                .max(0.0) as u32
        };
        let x = scale_component(pos.x);
        let y = scale_component(pos.y);
        let z = scale_component(pos.z);
        return usize::try_from(
            scale_component(pos.x)
                + (scale_component(pos.y) * self.cells_per_row)
                + (scale_component(pos.z) * self.cells_per_row * self.cells_per_row),
        )
        .unwrap();
    }
}
