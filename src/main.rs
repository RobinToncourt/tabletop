//#![allow(dead_code, unused, unused_imports)]

use bevy::{asset::AssetMetaCheck, input::mouse::AccumulatedMouseScroll, prelude::*};

const CARD_IMAGES: &[&str] = &[
    // ♠️ Spades
    "ace_of_spades.png",
    "2_of_spades.png",
    "3_of_spades.png",
    "4_of_spades.png",
    "5_of_spades.png",
    "6_of_spades.png",
    "7_of_spades.png",
    "8_of_spades.png",
    "9_of_spades.png",
    "10_of_spades.png",
    "jack_of_spades2.png",
    "queen_of_spades2.png",
    "king_of_spades2.png",
    // ♦️ Diamonds
    "ace_of_diamonds.png",
    "2_of_diamonds.png",
    "3_of_diamonds.png",
    "4_of_diamonds.png",
    "5_of_diamonds.png",
    "6_of_diamonds.png",
    "7_of_diamonds.png",
    "8_of_diamonds.png",
    "9_of_diamonds.png",
    "10_of_diamonds.png",
    "jack_of_diamonds2.png",
    "queen_of_diamonds2.png",
    "king_of_diamonds2.png",
    // ♣️ Clubs
    "ace_of_clubs.png",
    "2_of_clubs.png",
    "3_of_clubs.png",
    "4_of_clubs.png",
    "5_of_clubs.png",
    "6_of_clubs.png",
    "7_of_clubs.png",
    "8_of_clubs.png",
    "9_of_clubs.png",
    "10_of_clubs.png",
    "jack_of_clubs2.png",
    "queen_of_clubs2.png",
    "king_of_clubs2.png",
    // ♥️ Hearts
    "ace_of_hearts.png",
    "2_of_hearts.png",
    "3_of_hearts.png",
    "4_of_hearts.png",
    "5_of_hearts.png",
    "6_of_hearts.png",
    "7_of_hearts.png",
    "8_of_hearts.png",
    "9_of_hearts.png",
    "10_of_hearts.png",
    "jack_of_hearts2.png",
    "queen_of_hearts2.png",
    "king_of_hearts2.png",
    // 🃏 Jokers
    "black_joker.png",
    "red_joker.png",
];
const CARD_SIZE: Vec2 = Vec2::new(500.0, 726.0);

const SCROLL_FACTOR: f32 = if cfg!(target_arch = "wasm32") {
    0.2
} else {
    10.0
};

#[derive(Resource)]
struct DoubleClickTimer(Timer);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(DoubleClickTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (zoom_in, move_camera))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = CARD_SIZE / 10.0;

    commands.spawn(Camera2d);

    let start_x_offset = -325.0;
    let start_y_offset = 108.9;

    // Spawn cards.
    let mut y_pos = -1.0;
    for (i, card) in CARD_IMAGES.iter().enumerate() {
        let x_pos = i as f32 % 13.0;
        if x_pos == 0.0 {
            y_pos += 1.0;
        }

        let sprite_path = format!("cards/{card}");
        let sprite = Sprite {
            image: asset_server.load(&sprite_path),
            custom_size: Some(sprite_size),
            ..default()
        };
        let transform = Transform::from_xyz(
            start_x_offset + x_pos * sprite_size.x,
            start_y_offset - y_pos * sprite_size.y,
            0.0,
        );

        commands
            .spawn((sprite, Pickable::default(), transform))
            .observe(mouse_action_start)
            .observe(mouse_action)
            .observe(mouse_action_end);
    }

    // Spanw card back.
    let sprite_path = format!("card_back.png");
    let sprite = Sprite {
        image: asset_server.load(&sprite_path),
        custom_size: Some(sprite_size),
        ..default()
    };
    let transform = Transform::from_xyz(
        start_x_offset + 2.0 * sprite_size.x,
        start_y_offset - 4.0 * sprite_size.y,
        0.0,
    );

    commands
        .spawn((sprite, Pickable::default(), transform))
        .observe(mouse_action_start)
        .observe(mouse_action)
        .observe(mouse_action_end);
}

fn mouse_action_start(
    drag_start: On<Pointer<DragStart>>,
    mut query: Query<&mut Transform>,
    time: Res<Time>,
    mut double_click_timer: ResMut<DoubleClickTimer>,
) {
    let button = drag_start.event().event.button;
    if !matches!(button, PointerButton::Primary) {
        return;
    }

    if double_click_timer.0.tick(time.delta()).just_finished() {
        println!("double click");
    }

    if let Ok(mut transform) = query.get_mut(drag_start.event_target()) {
        transform.translation.z += 100.0;
    }
}

fn mouse_action(
    on_drag: On<Pointer<Drag>>,
    query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let button = on_drag.event().event.button;
    match button {
        PointerButton::Primary => drag(on_drag, query, camera, windows),
        PointerButton::Secondary => rotate(),
        PointerButton::Middle => {}
    }
}

fn drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera;

    let transform = query.get_mut(on_drag.event_target());
    let position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    if let (Ok(mut transform), Some(position)) = (transform, position) {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn rotate() {}

fn mouse_action_end(drag_end: On<Pointer<DragEnd>>, mut query: Query<&mut Transform>) {
    let button = drag_end.event().event.button;
    if !matches!(button, PointerButton::Primary) {
        return;
    }

    if let Ok(mut transform) = query.get_mut(drag_end.event_target()) {
        transform.translation.z -= 100.0;
    }
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

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    camera.translation.x += direction.x;
    camera.translation.y += direction.y;
}
