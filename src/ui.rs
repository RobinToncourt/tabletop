use bevy::prelude::*;

use bevy_egui::egui::Visuals;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_framepace::{FramepaceSettings, Limiter};

const INSTRUCTIONS: &str = r#"ZQSD to move camera
A and E to rotate camera
You can zoom with the wheel
Left click on card to move it around
Right to rotate it"#;

const LIGHT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DARK_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

#[derive(Component)]
struct ChangeBackgroundButton;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(ClearColor(LIGHT_COLOR))
            .add_systems(Startup, spawn_ui)
            .add_systems(EguiPrimaryContextPass, egui_system);
    }
}

fn spawn_ui(mut commands: Commands) {
    // Spawn instructions text, top left.
    commands.spawn((
        Text::new(INSTRUCTIONS),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(35.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));
}

fn egui_system(
    mut contexts: EguiContexts,
    mut background_color: ResMut<ClearColor>,
    mut framepace_settings: ResMut<FramepaceSettings>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let is_light = background_color.0 == LIGHT_COLOR;
    ctx.set_visuals(if is_light {
        Visuals::light()
    } else {
        Visuals::dark()
    });

    let button_label = if is_light { "🌙" } else { "☀" };

    let current_fps_limiter = if let Limiter::Manual(duration) = framepace_settings.limiter {
        Some(duration.as_secs_f64().recip())
    } else {
        None
    };

    let custom_frame = egui::Frame::default()
        .fill(ctx.style().visuals.panel_fill)
        .inner_margin(8.0);

    let mut change_background = false;

    egui::TopBottomPanel::top("menu_panel")
        .frame(custom_frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let styled_text = egui::RichText::new(button_label);
                let custom_button = egui::Button::new(styled_text);
                change_background = ui.add(custom_button).clicked();

                ui.separator();

                if let Some(mut current_fps_limiter) = current_fps_limiter {
                    ui.add(egui::Label::new(egui::RichText::new("FPS limit:")));
                    let slider_text = egui::RichText::new("fps");
                    let slider = ui.add(
                        egui::Slider::new(&mut current_fps_limiter, 15.0..=144.0).text(slider_text),
                    );

                    if slider.changed() {
                        framepace_settings.limiter = Limiter::from_framerate(current_fps_limiter);
                    }
                }
            });
        });

    if change_background {
        if background_color.0 == LIGHT_COLOR {
            background_color.0 = DARK_COLOR;
        } else {
            background_color.0 = LIGHT_COLOR;
        }
    }

    Ok(())
}
