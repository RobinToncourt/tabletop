//#![allow(dead_code, unused, unused_imports)]

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

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

#[cfg(not(target_arch = "wasm32"))]
const SCROLL_FACTOR: f32 = 10.0;

#[cfg(target_arch = "wasm32")]
const SCROLL_FACTOR: f32 = 0.2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (zoom_in, move_camera))
        .run();
}

#[derive(Component)]
struct Card;

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
            .spawn((sprite, Card, Pickable::default(), transform))
            .observe(card_drag_start)
            .observe(card_drag)
            .observe(card_drag_end);
    }
}

fn card_drag_start(on_drag_start: On<Pointer<DragStart>>, mut query: Query<&mut GlobalZIndex>) {
    if let Ok(mut global_zindex) = query.get_mut(on_drag_start.event_target()) {
        global_zindex.0 += 100;
    }
}

fn card_drag(
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

fn card_drag_end(on_drag_start: On<Pointer<DragEnd>>, mut query: Query<&mut GlobalZIndex>) {
    if let Ok(mut global_zindex) = query.get_mut(on_drag_start.event_target()) {
        global_zindex.0 -= 100;
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
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Card>)>,
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
