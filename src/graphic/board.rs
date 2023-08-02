use crate::graphic::config;

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
        config::MAJOR_LINE_THICKNESS
    } else {
        config::MINOR_LINE_THICKNESS
    };

    let length = config::GRID_SIZE + thickness;

    let size = match orientation {
        Orientation::Horizontal => Vec2::new(length.into(), thickness.into()),
        Orientation::Vertical => Vec2::new(thickness.into(), length.into()),
    };

    // Each object's position is defined by it's center
    let offset = i as f32 * config::CELL_SIZE;

    let (x, y) = match orientation {
        Orientation::Horizontal => (
            config::GRID_LEFT_EDGE + 0.5 * config::GRID_SIZE,
            config::GRID_BOT_EDGE + offset,
        ),
        Orientation::Vertical => (
            config::GRID_LEFT_EDGE + offset,
            config::GRID_BOT_EDGE + 0.5 * config::GRID_SIZE,
        ),
    };

    SpriteBundle {
        sprite: Sprite {
            color: config::BACKGROUND_COLOR,
            custom_size: Some(size),
            ..default()
        },
        // We want these grid lines to cover any cell that it might overlap with
        transform: Transform::from_xyz(x, y, 1.0),
        // material: grid_handle,
        ..Default::default()
    }
}
