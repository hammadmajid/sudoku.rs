use crate::graphic::config;

use bevy::prelude::*;

pub fn spawn_grid(mut commands: Commands, mut meterials: ResMut<Assets<ColorMaterial>>) {
    let grid_handle = meterials.add(config::BACKGROUND_COLOR.into());

    for row in 1..=9 {
        commands.spawn(new_gridline(
            Orientation::Horizontal,
            row,
            grid_handle.clone(),
        ));
    }
    for column in 1..=9 {
        commands.spawn(new_gridline(
            Orientation::Vertical,
            column,
            grid_handle.clone(),
        ));
    }
}

enum Orientation {
    Vertical,
    Horizontal,
}

fn new_gridline(
    orientation: Orientation,
    i: u8,
    grid_handle: Handle<ColorMaterial>,
) -> SpriteBundle {
    let thickness = if (i % 3) == 0 {
        config::MAJOR_LINE_THICKNESS
    } else {
        config::MINOR_LINE_THICKNESS
    };

    let length = config::GRID_SIZE + thickness;

    let size = match orientation {
        Orientation::Horizontal => Vec2::new(length, thickness),
        Orientation::Vertical => Vec2::new(thickness, length),
    };

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
        sprite: Sprite::default(),
        // We want these grid lines to cover any cell that it might overlap with
        transform: Transform::from_xyz(x, y, 1.0),
        // material: grid_handle,
        ..Default::default()
    }
}