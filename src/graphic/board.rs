use crate::graphic::config::*;
use crate::logic::board::*;

use bevy::prelude::*;

/// Spawn the sudoku board grid
pub fn spawn_grid(mut commands: Commands) {
    for row in 1..=9 {
        commands.spawn(new_gridline(Orientation::Horizontal, row));
    }
    for column in 1..=9 {
        commands.spawn(new_gridline(Orientation::Vertical, column));
    }
}

/// The orientation of the line on board
enum Orientation {
    /// For columns
    Vertical,
    /// For rows
    Horizontal,
}

/// Draws a new grid line with provided parameters
/// @returns SpriteBundle for Bevyengine to draw
fn new_gridline(orientation: Orientation, i: u8) -> SpriteBundle {
    // The grid lines that define the boxes need to be thicker
    let thickness = if (i % 3) == 0 {
        MAJOR_LINE_THICKNESS
    } else {
        MINOR_LINE_THICKNESS
    };

    let length = GRID_SIZE + thickness;

    let size = match orientation {
        Orientation::Horizontal => Vec2::new(length, thickness),
        Orientation::Vertical => Vec2::new(thickness, length),
    };

    // Each object's position is defined by it's center
    let offset = i as f32 * CELL_SIZE;

    let (x, y) = match orientation {
        Orientation::Horizontal => (GRID_LEFT_EDGE + 0.5 * GRID_SIZE, GRID_BOT_EDGE + offset),
        Orientation::Vertical => (GRID_LEFT_EDGE + offset, GRID_BOT_EDGE + 0.5 * GRID_SIZE),
    };

    SpriteBundle {
        sprite: Sprite {
            color: GRID_LINE_COLOR,
            custom_size: Some(size),
            ..default()
        },
        // We want these grid lines to cover any cell that it might overlap with
        transform: Transform::from_xyz(x, y, 1.0),
        // material: grid_handle,
        ..Default::default()
    }
}

/// Spawn all 81 cells on grid
pub fn spawn_cell(mut commands: Commands) {
    for row in 1..=9 {
        for column in 1..=9 {
            commands.spawn(CellBundle::new(row, column));
        }
    }
}

#[derive(Bundle)]
struct CellBundle {
    position: Position,
    value: Value,
    cell_fill: SpriteBundle,
}

impl CellBundle {
    fn new(row: u8, column: u8) -> Self {
        let x = GRID_LEFT_EDGE + CELL_SIZE * row as f32 - 0.5 * CELL_SIZE;
        let y = GRID_BOT_EDGE + CELL_SIZE * column as f32 - 0.5 * CELL_SIZE;

        CellBundle {
            position: Position {
                row,
                column,
                block: Position::compute_square(row, column),
            },
            value: Value::Empty,
            cell_fill: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: CELL_SIZE,
                        y: CELL_SIZE,
                    }),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
        }
    }
}
