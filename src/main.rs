#![allow(dead_code, unused, unused_imports)]

use std::fmt::Debug;

use bevy::{camera::ScalingMode, input::mouse::AccumulatedMouseScroll, prelude::*};
use bevy::window::PrimaryWindow;

macro_rules! unwrap_or_return {
    (ok $result:expr) => {
        {
            match $result {
                Ok(ok) => ok,
                Err(_) => return,
            }
        }
    };
    (some $option:expr) => {
        {
            match $option {
                Some(some) => some,
                None => return,
            }
        }
    }
}

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldCursorPosition>()
        .add_systems(Startup, setup)
        .add_systems(Update, zoom_in)
        .add_observer(move_card)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct WorldCursorPosition(Option<Vec2>);

#[derive(Component)]
struct Card;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = CARD_SIZE / 10.0;

    commands.spawn((Camera2d, MainCamera));

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

        commands.spawn((
            Sprite {
                image: asset_server.load(&sprite_path),
                custom_size: Some(sprite_size),
                ..default()
            },
            Card,
            Pickable::default(),
            Transform::from_xyz(start_x_offset + x_pos * sprite_size.x, start_y_offset - y_pos * sprite_size.y, 0.0),
        ));
    }
}

fn move_card(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut Transform, With<Card>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let transform = query.get_mut(on_drag.event_target());
    let (camera, camera_transform) = unwrap_or_return!(ok q_camera.single());
    let window = unwrap_or_return!(ok q_window.single()).cursor_position();

    if let (Ok(mut transform), Some(screen_pos)) = (transform, window) {
        if let Ok(position) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn zoom_in(
    camera: Single<&mut Projection, With<MainCamera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            let mut log_scale = orthographic.scale.ln();
            log_scale -= mouse_wheel_input.delta.y * time.delta_secs() * 10.0;
            orthographic.scale = log_scale.exp();
        }
        _ => (),
    }
}
