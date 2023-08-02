use bevy::prelude::*;

// Colors
#[allow(dead_code)]
pub const BACKGROUND_COLOR: Color = Color::rgb(30.0, 30.0, 46.0);
pub const GRID_LINE_COLOR: Color = Color::rgb(108.0, 112.0, 134.0);

#[allow(dead_code)]
pub const CELL_TEXT_COLOR: Color = Color::rgb(180.0, 190.0, 254.0);
#[allow(dead_code)]
pub const FIXED_CELL_BACKGROUND_COLOR: Color = Color::rgb(147.0, 153.0, 178.0);

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
#[allow(dead_code)]
pub const GRID_BOT_EDGE: f32 = GRID_CENTER_Y - 0.5 * GRID_SIZE;

#[allow(dead_code)]
pub const NUM_OFFSET_X: f32 = 0.0 * CELL_SIZE;
#[allow(dead_code)]
pub const NUM_OFFSET_Y: f32 = 0.03 * CELL_SIZE;
