use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_cube)
        .run();
}

#[derive(Component)]
struct Movable;

fn setup(mut commands: Commands) {
    // Adiciona uma câmera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Adiciona um cubo
    commands.spawn((
        PbrBundle {
            mesh: Mesh::from(shape::Cube { size: 1.0 }),
            material: StandardMaterial {
                base_color: Color::rgb(0.2, 0.5, 1.0),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Movable, // Componente que indica que o cubo pode se mover
    ));

    // Adiciona uma luz
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn move_cube(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    let speed = 0.1;
    
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.z -= speed;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.z += speed;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += speed;
        }
    }
}
