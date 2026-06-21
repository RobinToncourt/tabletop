#![allow(clippy::needless_pass_by_value)]

mod camera;
mod debug;
mod items;
mod socket;
mod ui;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use bevy_framepace::FramepacePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                // This is so the wasm window fit the browser page.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                // This is so it doesn't try to fetch .meta files for assets.
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            FramepacePlugin,
            items::ItemsPlugin,
            camera::CameraPlugin,
            debug::DebugPlugin,
            ui::UiPlugin,
            // socket::SocketPlugin,
        ))
        .run();
}
