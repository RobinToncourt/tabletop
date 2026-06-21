use bevy::{
    input::{common_conditions::input_pressed, mouse::AccumulatedMouseScroll},
    prelude::*,
};

/// Speed for camera movements.
const SPEED_FACTOR: f32 = 60.0;
const SCROLL_FACTOR: f32 = if cfg!(target_arch = "wasm32") {
    0.2
} else {
    10.0
};
const ROTATION_FACTOR: f32 = std::f32::consts::PI / 32.0 * SPEED_FACTOR;
const MOVEMENT_FACTOR: f32 = 10.0 * SPEED_FACTOR;

#[derive(Resource)]
pub struct CameraRotationAngle(pub f32);

#[derive(Resource)]
struct ZoomLevel(f32);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraRotationAngle(0.0))
            .insert_resource(ZoomLevel(MOVEMENT_FACTOR))
            .add_systems(Startup, camera_setup)
            .add_systems(
                Update,
                (
                    zoom_in.run_if(has_wheel_moved),
                    // KeyCode::KeyW = physical W position (QWERTY); shows as Z on AZERTY
                    move_camera_up.run_if(input_pressed(KeyCode::KeyW)),
                    move_camera_down.run_if(input_pressed(KeyCode::KeyS)),
                    move_camera_right.run_if(input_pressed(KeyCode::KeyD)),
                    move_camera_left.run_if(input_pressed(KeyCode::KeyA)),
                    rotate_camera_left.run_if(input_pressed(KeyCode::KeyQ)),
                    rotate_camera_right.run_if(input_pressed(KeyCode::KeyE)),
                ),
            );
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn has_wheel_moved(mouse_wheel_input: Res<AccumulatedMouseScroll>) -> bool {
    mouse_wheel_input.delta.y != 0.0
}

fn zoom_in(
    camera: Single<&mut Projection>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    mut zoom_level: ResMut<ZoomLevel>,
) {
    if let Projection::Orthographic(ref mut orthographic) = *camera.into_inner() {
        let mut log_scale = orthographic.scale.ln();
        log_scale -= mouse_wheel_input.delta.y * time.delta_secs() * SCROLL_FACTOR;
        let log_scale = log_scale.exp();
        orthographic.scale = log_scale;
        zoom_level.0 = MOVEMENT_FACTOR * log_scale;
    }
}

/*
 * Camera movements.
 */

fn move_camera_up(
    camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    zoom_level: Res<ZoomLevel>,
) {
    let forward = camera.rotation * Vec3::Y;
    move_camera(camera, forward, zoom_level, time);
}

fn move_camera_down(
    camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    zoom_level: Res<ZoomLevel>,
) {
    let forward = camera.rotation * Vec3::Y;
    move_camera(camera, -forward, zoom_level, time);
}

fn move_camera_right(
    camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    zoom_level: Res<ZoomLevel>,
) {
    let right = camera.rotation * Vec3::X;
    move_camera(camera, right, zoom_level, time);
}

fn move_camera_left(
    camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    zoom_level: Res<ZoomLevel>,
) {
    let right = camera.rotation * Vec3::X;
    move_camera(camera, -right, zoom_level, time);
}

fn move_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    movement: Vec3,
    zoom_level: Res<ZoomLevel>,
    time: Res<Time>,
) {
    camera.translation += movement * time.delta_secs() * zoom_level.0;
}

/*
 * Camera rotation.
 */

fn rotate_camera_left(
    camera: Single<&mut Transform, With<Camera2d>>,
    camera_rotation_angle: ResMut<CameraRotationAngle>,
    time: Res<Time>,
) {
    rotate_camera(camera, camera_rotation_angle, time, ROTATION_FACTOR);
}

fn rotate_camera_right(
    camera: Single<&mut Transform, With<Camera2d>>,
    camera_rotation_angle: ResMut<CameraRotationAngle>,
    time: Res<Time>,
) {
    rotate_camera(camera, camera_rotation_angle, time, -ROTATION_FACTOR);
}

fn rotate_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut camera_rotation_angle: ResMut<CameraRotationAngle>,
    time: Res<Time>,
    rotation_angle: f32,
) {
    camera_rotation_angle.0 += rotation_angle * time.delta_secs();
    camera.rotation = Quat::from_rotation_z(camera_rotation_angle.0);
}
