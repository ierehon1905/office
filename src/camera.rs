use bevy::prelude::*;

#[derive(Default)]
pub struct CustomCameraPlugin {}

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn camera_movement(mut camera: Query<&mut Transform, With<Camera>>, keys: Res<Input<KeyCode>>) {
    let pos = &mut camera.single_mut().translation;
    let mut velocity = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        velocity.y += 1.0;
    } else if keys.pressed(KeyCode::S) {
        velocity.y -= 1.0;
    };
    if keys.pressed(KeyCode::A) {
        velocity.x -= 1.0;
    } else if keys.pressed(KeyCode::D) {
        velocity.x += 1.0;
    };

    *pos += velocity.clamp_length_max(1.0) * 10.0;
}
