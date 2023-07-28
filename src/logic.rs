#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Value(u8),
    Empty,
}

pub struct Board {
    pub cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; 9]; 9],
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Cell {
        self.cells[row][column]
    }

    pub fn set_cell(&mut self, row: usize, column: usize, value: u8) {
        self.cells[row][column] = Cell::Value(value);
    }

    pub fn get_row(&self, row: usize) -> &[Cell] {
        &self.cells[row]
    }

    // TODO: fix E0515
    // fn get_column(&self, column: usize) -> &[Cell] {
    //     &self.cells.iter().map(|row| row[column]).collect::<Vec<_>>()
    // }

    // TODO: fix E0599
    // fn get_block(&self, row: usize, column: usize) -> &[Cell] {
    //     let block_row = row / 3;
    //     let block_column = column / 3;

    //     &Iterator::map::<_, _>(self.cells[block_row * 3..block_row * 3 + 3].iter(), |row| {
    //         row[block_column * 3..block_column * 3 + 3]
    //     })
    //     .flatten()
    // }
}
