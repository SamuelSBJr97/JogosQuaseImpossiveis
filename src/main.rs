use bevy::prelude::*;
mod components;
mod systems;
mod states;
mod resources;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::custom_plugin::CustomPlugin)
        .add_startup_system(systems::setup::setup)
        .add_state(states::GameState::Menu)
        .add_system_set(SystemSet::on_update(states::GameState::Gameplay)
            .with_system(systems::player_movement::player_movement)
            .with_system(systems::enemy_ai::enemy_ai))
        .run();
}