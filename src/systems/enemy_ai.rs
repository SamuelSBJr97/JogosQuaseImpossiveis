use bevy::prelude::*;
use crate::components::enemy::Enemy;

pub fn enemy_ai(mut query: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {
    for (enemy, mut transform) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 0.5; // Movimento simples
    }
}