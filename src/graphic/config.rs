use bevy::prelude::*;

// Colors
pub const BACKGROUND_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub const FILLED_CELL_COLOR: Color = Color::rgb(2.0, 2.0, 3.0);
pub const FIXED_CELL_COLOR: Color = Color::rgb(2.0, 2.0, 3.0);

// Sizes
pub const CELL_SIZE: f32 = 50.0;
pub const GRID_SIZE: f32 = 9.0 * CELL_SIZE;
pub const MINOR_LINE_THICKNESS: f32 = 2.0;
pub const MAJOR_LINE_THICKNESS: f32 = 4.0;

// Positions
// Defines the center lines of the grid in absolute coordinates
// (0, 0) is in the center of the screen in Bevy
pub const GRID_CENTER_X: f32 = 0.0;
pub const GRID_LEFT_EDGE: f32 = GRID_CENTER_X - 0.5 * GRID_SIZE;
pub const GRID_CENTER_Y: f32 = 0.0;
pub const GRID_BOT_EDGE: f32 = GRID_CENTER_Y - 0.5 * GRID_SIZE;

pub const NUM_OFFSET_X: f32 = 0.0 * CELL_SIZE;
pub const NUM_OFFSET_Y: f32 = 0.03 * CELL_SIZE;
