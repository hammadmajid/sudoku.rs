use bevy::prelude::*;

/// Cell's position on 9x9 board
#[derive(Component)]
#[allow(dead_code)]
pub struct Position {
    /// Between 1 and 9, counted from top to bottom
    row: u8,
    /// Between 1 and 9, counted from top to bottom
    column: u8,
    /// Squares are counted from 1 to 9 starting at the top left,
    /// in standard left-to-right reading order
    block: u8,
}

#[allow(dead_code)]
impl Position {
    /// Computes which 3x3 square a cell is in based on its row and column
    pub fn compute_square(row: u8, column: u8) -> u8 {
        const WIDTH: u8 = 3;
        let major_row = (row - 1) / WIDTH;
        let major_col = (column - 1) / WIDTH;

        major_col + major_row * WIDTH + 1
    }
}

/// The number(s) marked inside of each cell
#[derive(Component)]
#[allow(dead_code)]
pub enum Value {
    /// A single value is known to be in this cell
    Filled(u8),
    /// The value is provided by the puzzle and cannot be changed
    Fixed(u8),
    /// No value is filled in this cell
    Empty,
}
