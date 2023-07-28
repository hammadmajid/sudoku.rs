mod board;
mod logic;

use bevy::prelude::*;

fn main() {
    let mut sudoku_board = logic::Board::new();
    sudoku_board.set_cell(2, 2, 6);
    let row_2 = sudoku_board.get_row(2);

    println!("values of row 2: {:?}", row_2);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, board::spawn_board)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
