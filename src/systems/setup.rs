use bevy::prelude::*;
use crate::components::player::Player;

pub fn setup(mut commands: Commands) {
    // Adiciona um jogador
    commands.spawn((
        Player { speed: 5.0 },
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Adiciona uma câmera
    commands.spawn(Camera3dBundle::default());
}