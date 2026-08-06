use bevy::prelude::*;
use game_server_structs::websocket_communication::ServerMessage;
use shared::{AppState, MatchStateResource, set_title};
#[derive(Resource, Clone)]
pub struct InQueue(pub bool);
#[derive(Resource, Clone)]
pub struct InMatch(pub bool);
pub fn match_found(mut in_match: ResMut<InMatch>, mut server_events: MessageReader<ServerMessage>) {
    if in_match.0 {
        debug!("Received match_found message despite already being in a match");
        return;
    }
    for msg in server_events.read() {
        let ServerMessage::MatchFound {
            match_url,
            player_token,
            team,
        } = &msg
        else {
            continue;
        };
        debug!("MatchFound: Url: {match_url}, player_token: {player_token}, team: {team}");
        *in_match = InMatch(true);
        break;
    }
}

pub struct MatchPlugin;

impl Plugin for MatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Match), init_match)
            .add_systems(Update, update_match.run_if(in_state(AppState::Match)));
    }
}
pub fn init_match(mut windows: Query<&mut Window>, connection_info: Res<MatchStateResource>) {
    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut window, "In Match");
        window.resolution.set(960.0, 640.0);
    };
    match &*connection_info {
        MatchStateResource::NotInMatch => debug!("Not in match"),
        MatchStateResource::WantsToConnect {
            address,
            player_token,
        } => debug!(
            "Init_match: connecting to address: {} using player token: {}",
            address.0, player_token.0
        ),
    }
}

pub fn update_match() {}
