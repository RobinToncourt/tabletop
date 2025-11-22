use bevy::prelude::*;

const INSTRUCTIONS: &str = "ZQSD/arrows to move camera\nA and E to rotate camera\nYou can zoom with the wheel\nLeft clic on card to move it arround\nRight to rotate it";

const LIGHT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DARK_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

#[derive(Component)]
struct ChangeBackgroundButton;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(LIGHT_COLOR))
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, button_change_background);
    }
}

fn spawn_ui(mut commands: Commands) {
    // Spawn instrctions text, top left.
    commands.spawn((
        Text::new(INSTRUCTIONS),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(5),
            ..default()
        },
    ));

    // Spawn button to change background color, top right.
    commands.spawn((
        Button,
        ChangeBackgroundButton,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            position_type: PositionType::Absolute,
            top: px(5),
            right: px(5),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BorderRadius::MAX,
        BackgroundColor(Color::BLACK),
        children![(
            Text::new("Change\nbackground color"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 1.0)),
        )],
    ));
}

/// Inverse the background color on `ChangeBackgroundButton` button click.
fn button_change_background(
    mut background_color: ResMut<ClearColor>,
    interaction_query: Single<&Interaction, (With<ChangeBackgroundButton>, Changed<Interaction>)>,
) {
    match **interaction_query {
        Interaction::Pressed => {
            if background_color.0 == LIGHT_COLOR {
                background_color.0 = DARK_COLOR;
            } else {
                background_color.0 = LIGHT_COLOR;
            }
        }
        _ => {}
    }
}
