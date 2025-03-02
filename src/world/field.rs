use std::iter::Enumerate;

use crate::utility::Position;

pub struct Field<FieldType> {
    data: Vec<FieldType>,
    grid_size: u32,
    size: f32,
    scale: f32,
}

impl<FieldType: Copy + std::ops::AddAssign> Field<FieldType> {
    pub fn new(size: f32, divisions: u32, default_value: FieldType) -> Field<FieldType> {
        let mut grid_size = divisions;
        if grid_size % 2 == 0 {
            grid_size += 1;
        }
        let mut field = Field {
            data: Vec::new(),
            grid_size,
            size,
            scale: ((grid_size - 1) as f32) / (2.0 * size),
        };
        let vec_size = usize::try_from(divisions.pow(3)).unwrap();
        field.data.resize(vec_size, default_value);
        field.data.shrink_to_fit();
        return field;
    }

    pub fn field_value(&self, pos: &Position) -> FieldType {
        self.data[self.index_from_position(pos)]
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

    pub fn position_from_index(&self, index: usize) -> Position {
        Position::new(0.0, 0.0, 0.0)
    }

    // !TODO: does this work? had a panic
    fn index_from_position(&self, pos: &Position) -> usize {
        let x = (((pos.x + self.size) * self.scale) + 0.5).floor() as u32;
        let y = (((pos.y + self.size) * self.scale) + 0.5).floor() as u32;
        let z = (((pos.z + self.size) * self.scale) + 0.5).floor() as u32;
        return usize::try_from(x + (y * self.grid_size) + (z * self.grid_size * self.grid_size))
            .unwrap();
    }
}
