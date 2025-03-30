use bevy::prelude::*;

pub fn menu_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Press SPACE to Start",
        TextStyle {
            font_size: 50.0,
            color: Color::WHITE,
            ..default()
        },
     ));
}