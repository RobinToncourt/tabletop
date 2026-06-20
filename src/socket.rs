use bevy::prelude::*;
use bevy_matchbox::prelude::*;

pub struct SocketPlugin;
impl Plugin for SocketPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, open_socket_system)
            .add_systems(Update, say_hello_system);
    }
}

#[derive(Resource)]
pub struct Socket(pub MatchboxSocket);

fn open_socket_system(mut commands: Commands) {
    let room_url = "ws://localhost:3536/";

    let socket: MatchboxSocket = WebRtcSocketBuilder::new(room_url)
        .add_channel(ChannelConfig::reliable())
        .into();

    commands.insert_resource(Socket(socket));
}

fn say_hello_system(mut socket: ResMut<Socket>) {
    let Ok(peers) = socket.0.try_update_peers() else {
        return;
    };

    for (peer, state) in peers {
        match state {
            PeerState::Connected => {
                info!("Peer '{peer}' joined.");
            }
            PeerState::Disconnected => {
                info!("Peer '{peer}' left.");
            }
        }
    }
}
