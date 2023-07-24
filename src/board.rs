use bevy::prelude::*;

pub fn spawn_board(mut commands: Commands) {
    let cell = Sprite {
        color: Color::rgb(12.0, 12.0, 12.0),
        custom_size: Some(Vec2::new(10.0, 12.0)),
        ..default()
    };

    let cell_size = 21.0;
    let grid_size = 9;
    let grid_width = cell_size * grid_size as f32;
    let grid_height = cell_size * grid_size as f32;

    for x in 0..grid_size {
        for y in 0..grid_size {
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * cell_size - grid_width / 2.0 + cell_size / 2.0,
                    y as f32 * cell_size - grid_height / 2.0 + cell_size / 2.0,
                    0.0,
                )),
                sprite: cell.clone(),
                ..Default::default()
            });
        }
    }
}
