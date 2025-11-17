use bevy::prelude::*;

const CURSOR_POSITION_STR: &str = "Cursor position: \nTo camera:";
const CAMERA_ROTATION_STR: &str = "Camera rotation:";

#[derive(Component)]
struct CursorPosDebug;

#[derive(Component)]
struct CameraRotationDebug;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_tools)
            .add_systems(Update, (debug_cursor_position, debug_camera_rotation));
    }
}

fn spawn_debug_tools(mut commands: Commands) {
    commands.spawn((
        CursorPosDebug,
        Text::new(CURSOR_POSITION_STR),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(5),
            ..default()
        },
    ));
    commands.spawn((
        CameraRotationDebug,
        Text::new(CAMERA_ROTATION_STR),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(10),
            right: px(10),
            ..default()
        },
    ));
}

/// Tracks the position of the cursor in the window.
fn debug_cursor_position(
    mut display: Single<&mut Text, With<CursorPosDebug>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera;

    let rel_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    if let (Some(abs_pos), Some(rel_pos)) = (window.cursor_position(), rel_pos) {
        display.0 = format!(
            "{CURSOR_POSITION_STR}\nx: {}\ny: {}\nTo world: \nx: {}\ny: {}",
            abs_pos.x, abs_pos.y, rel_pos.x, rel_pos.y,
        );
    }
}

fn debug_camera_rotation(
    mut display: Single<&mut Text, With<CameraRotationDebug>>,
    camera: Single<(&Transform, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = *camera;
    display.0 = format!(
        "Camera pos:\n{:#?}\n{CAMERA_ROTATION_STR}\n{:#?}",
        camera_transform.translation(),
        camera.rotation
    );
}
