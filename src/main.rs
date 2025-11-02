#![allow(dead_code, unused, unused_imports)]

use std::fmt::Debug;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CARD_SIZE: Vec2 = Vec2::new(500.0, 726.0);
const CARDS: [&str; 54] = [
    "10_of_clubs.png",
    "10_of_diamonds.png",
    "10_of_hearts.png",
    "10_of_spades.png",
    "2_of_clubs.png",
    "2_of_diamonds.png",
    "2_of_hearts.png",
    "2_of_spades.png",
    "3_of_clubs.png",
    "3_of_diamonds.png",
    "3_of_hearts.png",
    "3_of_spades.png",
    "4_of_clubs.png",
    "4_of_diamonds.png",
    "4_of_hearts.png",
    "4_of_spades.png",
    "5_of_clubs.png",
    "5_of_diamonds.png",
    "5_of_hearts.png",
    "5_of_spades.png",
    "6_of_clubs.png",
    "6_of_diamonds.png",
    "6_of_hearts.png",
    "6_of_spades.png",
    "7_of_clubs.png",
    "7_of_diamonds.png",
    "7_of_hearts.png",
    "7_of_spades.png",
    "8_of_clubs.png",
    "8_of_diamonds.png",
    "8_of_hearts.png",
    "8_of_spades.png",
    "9_of_clubs.png",
    "9_of_diamonds.png",
    "9_of_hearts.png",
    "9_of_spades.png",
    "ace_of_clubs.png",
    "ace_of_diamonds.png",
    "ace_of_hearts.png",
    "ace_of_spades.png",
    "black_joker.png",
    "jack_of_clubs2.png",
    "jack_of_diamonds2.png",
    "jack_of_hearts2.png",
    "jack_of_spades2.png",
    "king_of_clubs2.png",
    "king_of_diamonds2.png",
    "king_of_hearts2.png",
    "king_of_spades2.png",
    "queen_of_clubs2.png",
    "queen_of_diamonds2.png",
    "queen_of_hearts2.png",
    "queen_of_spades2.png",
    "red_joker.png",
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldCursorPosition>()
        .add_systems(Startup, setup)
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
    let ace_of_spades = asset_server.load("cards/ace_of_spades.png");
    let ace_of_diamonds = asset_server.load("cards/ace_of_diamonds.png");

    commands.spawn((Camera2d, MainCamera));

    let sprite_size = CARD_SIZE / 10.0;

    // Spawn cards.
    commands.spawn((
        Sprite {
            image: ace_of_diamonds,
            custom_size: Some(sprite_size),
            ..default()
        },
        Card,
        Pickable::default(),
        Transform::from_xyz(-250.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sprite {
            image: ace_of_spades,
            custom_size: Some(sprite_size),
            ..default()
        },
        Card,
        Pickable::default(),
    ));
}

fn move_card(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut Transform>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    info!("Dragging");
    let transform = query.get_mut(on_drag.event_target());
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap().cursor_position();

    if let (Ok(mut transform), Some(screen_pos)) = (transform, window) {
        if let Ok(position) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn move_cards_example(time: Res<Time>, mut cards: Query<&mut Transform, (With<Card>,)>) {
    let t = time.elapsed_secs() * 0.1;
    for mut transform in &mut cards {
        let new = Vec2 {
            x: 50.0 * ops::sin(t),
            y: 50.0 * ops::sin(t * 2.0),
        };
        transform.translation.x = new.x;
        transform.translation.y = new.y;
    }
}

/*
   . observe(*|over: On<Pointer<Over>>| {
   info!("Over");
   })
   .observe(|out: On<Pointer<Out>>| {
   info!("Out");
   })
   .observe(|press: On<Pointer<Press>>| {
   info!("Press");
   })
   .observe(|release: On<Pointer<Release>>| {
   info!("Release");
   })
   .observe(|on_drag_start: On<Pointer<DragStart>>| {
   info!("Drag start");
   })
   .observe(|on_drag: On<Pointer<Drag>>, query: Query<&mut Transform>| {
   move_card(on_drag, query);
   })
   .observe(|on_drag_end: On<Pointer<DragEnd>>| {
   info!("Drag end");
   })
   .observe(|on_drag_drop: On<Pointer<DragDrop>>| {
   info!("Drag drop");
   })
*/
