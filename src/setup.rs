use bevy::{asset::AssetMetaCheck, prelude::*};

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

#[derive(Component)]
pub struct Card;

#[derive(Resource)]
struct DistanceCursorCenter(Option<Vec2>);

#[derive(Resource)]
struct ItemZTransformValue(f32);

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DistanceCursorCenter(None))
            .add_plugins(
                DefaultPlugins
                    .build()
                    // This is so the wasm window fit the browser page.
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            fit_canvas_to_parent: true,
                            ..default()
                        }),
                        ..default()
                    })
                    // This is so it doesn't try to fetch .meta files for assets.
                    .set(AssetPlugin {
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    }),
            )
            .insert_resource(ItemZTransformValue(0.0))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    z_transform: ResMut<ItemZTransformValue>,
) {
    // spawn_chess(commands, asset_server, atlas_layouts, z_transform);
    spawn_cards(commands, asset_server);
}

fn spawn_chess(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut z_transform: ResMut<ItemZTransformValue>,
) {
    let chess_board = Sprite {
        image: asset_server.load("chess_board.png"),
        ..default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, z_transform.0);
    commands.spawn((chess_board, transform));
    z_transform.0 += 1.0;

    let pieces_texture = asset_server.load("ChessPiecesArray.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(60), 6, 2, None, None);
    let texture_atlas_handle = atlas_layouts.add(texture_atlas);

    let chess_board_size = (637.0 / 2.0, 636.0 / 2.0);

    const BLACK_BACKLANE: f32 = 0.8;
    const BLACK_FRONTLANE: f32 = 0.58;
    const WHITE_BACKLANE: f32 = -0.8;
    const WHITE_FRONTLANE: f32 = -0.58;

    const COL_A: f32 = -0.8;
    const COL_B: f32 = -0.6;
    const COL_C: f32 = -0.35;
    const COL_D: f32 = -0.13;

    const COL_E: f32 = 0.13;
    const COL_F: f32 = 0.35;
    const COL_G: f32 = 0.6;
    const COL_H: f32 = 0.8;

    let pieces_pos: &[(usize, f32, f32)] = &[
        (0, COL_E, BLACK_BACKLANE),
        (1, COL_D, BLACK_BACKLANE),
        (2, COL_A, BLACK_BACKLANE),
        (2, COL_H, BLACK_BACKLANE),
        (3, COL_B, BLACK_BACKLANE),
        (3, COL_G, BLACK_BACKLANE),
        (4, COL_C, BLACK_BACKLANE),
        (4, COL_F, BLACK_BACKLANE),
        (5, COL_A, BLACK_FRONTLANE),
        (5, COL_B, BLACK_FRONTLANE),
        (5, COL_C, BLACK_FRONTLANE),
        (5, COL_D, BLACK_FRONTLANE),
        (5, COL_E, BLACK_FRONTLANE),
        (5, COL_F, BLACK_FRONTLANE),
        (5, COL_G, BLACK_FRONTLANE),
        (5, COL_H, BLACK_FRONTLANE),
        (6, COL_E, WHITE_BACKLANE),
        (7, COL_D, WHITE_BACKLANE),
        (8, COL_A, WHITE_BACKLANE),
        (8, COL_H, WHITE_BACKLANE),
        (9, COL_B, WHITE_BACKLANE),
        (9, COL_G, WHITE_BACKLANE),
        (10, COL_C, WHITE_BACKLANE),
        (10, COL_F, WHITE_BACKLANE),
        (11, COL_A, WHITE_FRONTLANE),
        (11, COL_B, WHITE_FRONTLANE),
        (11, COL_C, WHITE_FRONTLANE),
        (11, COL_D, WHITE_FRONTLANE),
        (11, COL_E, WHITE_FRONTLANE),
        (11, COL_F, WHITE_FRONTLANE),
        (11, COL_G, WHITE_FRONTLANE),
        (11, COL_H, WHITE_FRONTLANE),
    ];

    for (z, x, y) in pieces_pos {
        let piece = Sprite::from_atlas_image(
            pieces_texture.clone(),
            TextureAtlas {
                layout: texture_atlas_handle.clone(),
                index: *z,
            },
        );
        let transform = Transform::from_xyz(
            *x * chess_board_size.0,
            *y * chess_board_size.1,
            z_transform.0,
        );
        spawn_draggable(&mut commands, (piece, Pickable::default(), transform));
        z_transform.0 += 1.0;
    }
}

fn spawn_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = CARD_SIZE / 10.0;

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
            i as f32,
        );

        commands
            .spawn((sprite, Pickable::default(), transform, Card))
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
        CARD_IMAGES.len() as f32,
    );

    commands
        .spawn((sprite, Pickable::default(), transform, Card))
        .observe(mouse_action_start)
        .observe(mouse_action)
        .observe(mouse_action_end);
}

fn spawn_draggable<T>(commands: &mut Commands, bundle: T)
where
    T: Bundle,
{
    commands
        .spawn(bundle)
        .observe(mouse_action_start)
        .observe(mouse_action)
        .observe(mouse_action_end);
}

fn mouse_action_start(
    drag_start: On<Pointer<DragStart>>,
    mut query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut distance_cursor_center: ResMut<DistanceCursorCenter>,
) {
    println!("mouse_action_start _ On<Pointer<DragStart>>");

    let button = drag_start.event().event.button;
    if !matches!(button, PointerButton::Primary) {
        return;
    }

    let (camera, camera_transform) = *camera;

    let position: Option<Vec2> = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    let mut max_z = {
        let (Ok(target_transform), Some(position)) =
            (query.get_mut(drag_start.event_target()), position)
        else {
            return;
        };
        //target_transform.translation.distance(position)
        distance_cursor_center.0 = Some(Vec2 {
            x: target_transform.translation.x - position.x,
            y: target_transform.translation.y - position.y,
        });
        target_transform.translation.z
    };

    for mut item in &mut query {
        if item.translation.z > max_z {
            if item.translation.z > max_z {
                max_z = item.translation.z;
            }
            item.translation.z += 1.0;
        }
    }

    let _ = query
        .get_mut(drag_start.event_target())
        .map(|mut target_transform| target_transform.translation.z = max_z);
}

fn mouse_action(
    on_drag: On<Pointer<Drag>>,
    query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    distance_cursor_center: Res<DistanceCursorCenter>,
) {
    let button = on_drag.event().event.button;
    match button {
        PointerButton::Primary => drag(on_drag, query, camera, window, distance_cursor_center),
        PointerButton::Secondary => rotate_item(on_drag, query, camera, window),
        PointerButton::Middle => {}
    }
}

fn drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    distance_cursor_center: Res<DistanceCursorCenter>,
) {
    let (camera, camera_transform) = *camera;

    let position: Option<Vec2> = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
    let transform = query.get_mut(on_drag.event_target());

    let distance_cursor_center = distance_cursor_center.0.unwrap_or(Vec2::ZERO);

    if let (Some(position), Ok(mut transform)) = (position, transform) {
        transform.translation.x = position.x + distance_cursor_center.x;
        transform.translation.y = position.y + distance_cursor_center.y;
    }
}

fn rotate_item(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut Transform>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera;
    let cursor_translation: Option<Vec2> = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
    let target_transform = query.get_mut(on_drag.event_target());

    if let (Some(cursor_translation), Ok(mut target_transform)) =
        (cursor_translation, target_transform)
    {
        let to_cursor = (cursor_translation - target_transform.translation.xy()).normalize();
        let rotate_to_cursor = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.0));
        target_transform.rotation = rotate_to_cursor;
    }
}

fn mouse_action_end(
    _drag_end: On<Pointer<DragEnd>>,
    mut distance_cursor_center: ResMut<DistanceCursorCenter>,
) {
    distance_cursor_center.0 = None;
}
