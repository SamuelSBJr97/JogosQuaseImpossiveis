use bevy::prelude::*;

pub fn gameplay_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}