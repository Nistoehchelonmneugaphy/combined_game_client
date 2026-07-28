use bevy::prelude::*;
use game_server_structs::websocket_communication::ServerMessage;
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
