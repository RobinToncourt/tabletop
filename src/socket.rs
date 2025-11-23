use bevy::prelude::*;
use bevy_matchbox::prelude::*;

pub struct SocketPlugin;
impl Plugin for SocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/tabletop?next=2";
    info!("Connecting to matchbox server: '{room_url}'.");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

fn wait_for_players(mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return; // We've already started.
    }

    // Check for new connections.
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // Wait for more players.
    }

    info!("All peers have joined, going in-game.");
}
