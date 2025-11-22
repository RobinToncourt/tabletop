use bevy::{
    input::{common_conditions::input_pressed, mouse::AccumulatedMouseScroll},
    prelude::*,
};

const SCROLL_FACTOR: f32 = if cfg!(target_arch = "wasm32") {
    0.2
} else {
    10.0
};
const ROTATION_FACTOR: f32 = std::f32::consts::PI / 32.0;

#[derive(Resource)]
pub struct CameraRotationAngle(pub f32);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraRotationAngle(0.0))
            .add_systems(Startup, camera_setup)
            .add_systems(
                Update,
                (
                    zoom_in.run_if(has_wheel_moved),
                    move_camera::<1, 0>.run_if(input_pressed(KeyCode::KeyD)),
                    move_camera::<-1, 0>.run_if(input_pressed(KeyCode::KeyA)),
                    move_camera::<0, 1>.run_if(input_pressed(KeyCode::KeyW)),
                    move_camera::<0, -1>.run_if(input_pressed(KeyCode::KeyS)),
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
) {
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            let mut log_scale = orthographic.scale.ln();
            log_scale -= mouse_wheel_input.delta.y * time.delta_secs() * SCROLL_FACTOR;
            orthographic.scale = log_scale.exp();
        }
        _ => (),
    }
}

fn move_camera<const X: i32, const Y: i32>(mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation.x += X as f32;
    camera.translation.y += Y as f32;
}

fn rotate_camera_left(
    camera: Single<&mut Transform, With<Camera2d>>,
    camera_rotation_angle: ResMut<CameraRotationAngle>,
) {
    rotate_camera(camera, camera_rotation_angle, ROTATION_FACTOR);
}

fn rotate_camera_right(
    camera: Single<&mut Transform, With<Camera2d>>,
    camera_rotation_angle: ResMut<CameraRotationAngle>,
) {
    rotate_camera(camera, camera_rotation_angle, -ROTATION_FACTOR);
}

fn rotate_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut camera_rotation_angle: ResMut<CameraRotationAngle>,
    rotation_angle: f32,
) {
    camera_rotation_angle.0 += rotation_angle;
    camera.rotation = Quat::from_rotation_z(camera_rotation_angle.0);
}
