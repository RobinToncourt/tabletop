use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

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

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<OutlineMaterial>::default())
            .insert_resource(LastItemZTransformValue(0.0))
            .add_systems(Startup, setup)
            .add_systems(Update, (add_outline, remove_outline));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct OutlineMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    color: LinearRgba,
    #[uniform(2)]
    region: Vec4, // (u_min, v_min, u_max, v_max) of the sprite within `texture`
    #[uniform(2)]
    outline_px: f32, // outline thickness, in source-texture texels
    #[uniform(2)]
    mesh_scale: f32, // how much bigger the outline mesh is than the sprite (e.g. 1.15)
}

impl Material2d for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
struct SelectionOutline;

#[derive(Component)]
struct CursorDistance(Vec3);

#[derive(Resource)]
struct LastItemZTransformValue(f32);
impl LastItemZTransformValue {
    fn get_then_increase(&mut self) -> f32 {
        let tmp = self.0;
        self.0 += 1.0;
        tmp
    }
}

fn add_outline(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    newly_selected: Query<(Entity, &Sprite), Added<Selected>>,
) {
    for (entity, sprite) in &newly_selected {
        let (region, base_size) = match &sprite.texture_atlas {
            Some(atlas) => {
                let layout = atlas_layouts.get(&atlas.layout).unwrap();
                let rect = layout.textures[atlas.index];
                let size = layout.size.as_vec2();
                (
                    Vec4::new(
                        rect.min.x as f32 / size.x,
                        rect.min.y as f32 / size.y,
                        rect.max.x as f32 / size.x,
                        rect.max.y as f32 / size.y,
                    ),
                    rect.size().as_vec2(),
                )
            }
            None => (
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                sprite.custom_size.unwrap_or(Vec2::splat(64.0)),
            ),
        };

        let mesh_scale = 1.15;
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SelectionOutline,
                Mesh2d(meshes.add(Rectangle::new(
                    base_size.x * mesh_scale,
                    base_size.y * mesh_scale,
                ))),
                MeshMaterial2d(materials.add(OutlineMaterial {
                    texture: sprite.image.clone(),
                    color: LinearRgba::rgb(1.0, 0.85, 0.1),
                    region,
                    outline_px: 5.0,
                    mesh_scale,
                })),
                Pickable::IGNORE,
            ));
        });
    }
}

fn remove_outline(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    children_query: Query<&Children>,
    outline_query: Query<Entity, With<SelectionOutline>>,
) {
    for entity in removed.read() {
        if let Ok(children) = children_query.get(entity) {
            for &child in children {
                if outline_query.contains(child) {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut z_transform: ResMut<LastItemZTransformValue>,
) {
    spawn_chess(
        &mut commands,
        &mut asset_server,
        atlas_layouts,
        &mut z_transform,
    );
    spawn_cards(&mut commands, &mut asset_server, &mut z_transform);
}

#[allow(clippy::items_after_statements)]
#[allow(dead_code)]
fn spawn_chess(
    mut commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    z_transform: &mut ResMut<LastItemZTransformValue>,
) {
    let chess_board = Sprite {
        image: asset_server.load("chess/chess_board.png"),
        ..default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, z_transform.get_then_increase());
    commands.spawn((chess_board, transform));

    let pieces_texture = asset_server.load("chess/chess_pieces.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(60), 6, 2, None, None);
    let texture_atlas_handle = atlas_layouts.add(texture_atlas);

    let chess_board_size = (637.0 / 2.0, 636.0 / 2.0);

    const BLACK_BACK_LANE: f32 = 0.8;
    const BLACK_FRONT_LANE: f32 = 0.58;
    const WHITE_BACK_LANE: f32 = -0.8;
    const WHITE_FRONT_LANE: f32 = -0.58;

    const COL_A: f32 = -0.8;
    const COL_B: f32 = -0.6;
    const COL_C: f32 = -0.35;
    const COL_D: f32 = -0.13;

    const COL_E: f32 = 0.13;
    const COL_F: f32 = 0.35;
    const COL_G: f32 = 0.6;
    const COL_H: f32 = 0.8;

    let pieces_pos: &[(usize, f32, f32)] = &[
        (0, COL_E, BLACK_BACK_LANE),
        (1, COL_D, BLACK_BACK_LANE),
        (2, COL_A, BLACK_BACK_LANE),
        (2, COL_H, BLACK_BACK_LANE),
        (3, COL_B, BLACK_BACK_LANE),
        (3, COL_G, BLACK_BACK_LANE),
        (4, COL_C, BLACK_BACK_LANE),
        (4, COL_F, BLACK_BACK_LANE),
        (5, COL_A, BLACK_FRONT_LANE),
        (5, COL_B, BLACK_FRONT_LANE),
        (5, COL_C, BLACK_FRONT_LANE),
        (5, COL_D, BLACK_FRONT_LANE),
        (5, COL_E, BLACK_FRONT_LANE),
        (5, COL_F, BLACK_FRONT_LANE),
        (5, COL_G, BLACK_FRONT_LANE),
        (5, COL_H, BLACK_FRONT_LANE),
        (6, COL_E, WHITE_BACK_LANE),
        (7, COL_D, WHITE_BACK_LANE),
        (8, COL_A, WHITE_BACK_LANE),
        (8, COL_H, WHITE_BACK_LANE),
        (9, COL_B, WHITE_BACK_LANE),
        (9, COL_G, WHITE_BACK_LANE),
        (10, COL_C, WHITE_BACK_LANE),
        (10, COL_F, WHITE_BACK_LANE),
        (11, COL_A, WHITE_FRONT_LANE),
        (11, COL_B, WHITE_FRONT_LANE),
        (11, COL_C, WHITE_FRONT_LANE),
        (11, COL_D, WHITE_FRONT_LANE),
        (11, COL_E, WHITE_FRONT_LANE),
        (11, COL_F, WHITE_FRONT_LANE),
        (11, COL_G, WHITE_FRONT_LANE),
        (11, COL_H, WHITE_FRONT_LANE),
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
            z_transform.get_then_increase(),
        );
        spawn_draggable(&mut commands, (piece, Pickable::default(), transform));
    }
}

fn spawn_cards(
    mut commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    z_transform: &mut ResMut<LastItemZTransformValue>,
) {
    let sprite_size = CARD_SIZE / 10.0;

    let start_x_offset = -1625.0;
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
            z_transform.get_then_increase(),
        );

        spawn_draggable(
            &mut commands,
            (sprite, Pickable::default(), transform),
        );
    }

    // Spawn card back.
    let sprite_path = "cards/card_back.png".to_owned();
    let sprite = Sprite {
        image: asset_server.load(&sprite_path),
        custom_size: Some(sprite_size),
        ..default()
    };
    let transform = Transform::from_xyz(
        start_x_offset + 2.0 * sprite_size.x,
        start_y_offset - 4.0 * sprite_size.y,
        z_transform.get_then_increase(),
    );

    spawn_draggable(
        &mut commands,
        (sprite, Pickable::default(), transform),
    );
}

fn spawn_draggable<T>(commands: &mut Commands, bundle: T)
where
    T: Bundle,
{
    commands
        .spawn(bundle)
        .observe(mouse_drag_start)
        .observe(mouse_drag)
        .observe(mouse_drag_end)
        .observe(mouse_press);
}

/// Called on click on an item.
fn mouse_press(
    press: On<Pointer<Press>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut all_selected: Query<Entity, With<Selected>>,
) {
    let clicked_entity = press.event_target();

    if keyboard.pressed(KeyCode::ControlLeft) {
        if all_selected.get_mut(clicked_entity).is_ok() {
            commands.entity(clicked_entity).remove::<Selected>();
        } else {
            commands.entity(clicked_entity).insert(Selected);
        }
    } else {
        for entity in all_selected {
            commands.entity(entity).remove::<Selected>();
        }
        commands.entity(clicked_entity).insert(Selected);
    }
}

fn get_cursor_position_in_world(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera;
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
}

fn mouse_drag_start(
    _drag_start: On<Pointer<DragStart>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    all_selected: Query<(Entity, &mut Transform), With<Selected>>,
    mut commands: Commands,
) {
    let cursor_position_in_world = get_cursor_position_in_world(camera, window);
    let Some(cursor_position_in_world) = cursor_position_in_world else {
        return;
    };

    // Add to all selected entity the distance to the cursor.
    for (entity, transform) in all_selected {
        let cursor_distance =
            CursorDistance(transform.translation - cursor_position_in_world.extend(0.0));
        commands.entity(entity).insert(cursor_distance);
    }

    // TODO: Put all the entities with `Selected` on top of the others.
}

fn mouse_drag(
    on_drag: On<Pointer<Drag>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    all_selected: Query<(&mut Transform, &CursorDistance), With<Selected>>,
) {
    let button = on_drag.event().event.button;
    match button {
        PointerButton::Primary => drag(camera, window, all_selected),
        PointerButton::Secondary => rotate_item(on_drag, camera, window, all_selected),
        PointerButton::Middle => {}
    }
}

fn drag(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    all_selected: Query<(&mut Transform, &CursorDistance), With<Selected>>,
) {
    let cursor_position_in_world = get_cursor_position_in_world(camera, window);
    let Some(cursor_position_in_world) = cursor_position_in_world else {
        return;
    };

    for (mut transform, cursor_distance) in all_selected {
        transform.translation = cursor_position_in_world.extend(0.0) + cursor_distance.0;
    }
}

fn rotate_item(
    on_drag: On<Pointer<Drag>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut query: Query<(&mut Transform, &CursorDistance), With<Selected>>,
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
        let to_cursor = (cursor_translation - target_transform.0.translation.xy()).normalize();
        let rotate_to_cursor = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.0));
        target_transform.0.rotation = rotate_to_cursor;
    }
}

fn mouse_drag_end(
    _drag_end: On<Pointer<DragEnd>>,
    all_cursor_distance: Query<Entity, With<CursorDistance>>,
    mut commands: Commands,
) {
    for entity in all_cursor_distance {
        commands.entity(entity).remove::<CursorDistance>();
    }
}
