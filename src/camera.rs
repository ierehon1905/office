use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(Default)]
pub struct CustomCameraPlugin {}

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        dbg!(CursorPosition::default());

        app.insert_resource(CursorPosition::NONE)
            .add_startup_system(setup_camera)
            .add_system(camera_movement)
            .add_system(my_cursor_system);
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

fn camera_movement(
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut pos, mut cam) = camera.single_mut();
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

    if keys.pressed(KeyCode::Q) {
        cam.scale -= 0.01 * cam.scale;
    } else if keys.pressed(KeyCode::E) {
        cam.scale += 0.01 * cam.scale;
    };
    cam.scale = cam.scale.clamp(0.32, 32.0);

    pos.translation += velocity.clamp_length_max(1.0) * 10.0;
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut pos: ResMut<CursorPosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

        pos.0 = Some(world_pos);
    } else {
        pos.0 = None;
    }
}

#[derive(Default, Debug)]
pub(crate) struct CursorPosition(pub(crate) Option<Vec2>);

impl CursorPosition {
    pub(crate) const ZERO: Self = Self(Some(Vec2::ZERO));
    pub(crate) const NONE: Self = Self(None);
}
