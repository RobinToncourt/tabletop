mod camera;
mod debug;
mod setup;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            setup::SetupPlugin,
            camera::CameraPlugin,
            debug::DebugPlugin,
            ui::UiPlugin,
        ))
        .run();
}
