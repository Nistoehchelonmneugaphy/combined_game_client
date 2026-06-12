use bevy::prelude::*;
use shared::AppConfig;

pub fn teardown_server_select_ui() {

}

pub fn setup_server_select_ui(app_config: Res<AppConfig>) {
    debug!("From setup_server_select_ui");
    let app_config = app_config.clone();
    // TODO maybe add proper server seleciton later, if I wanted
    // For now, just immediately go into AppState::Server
    debug!("Connecting to game server at {:#?} ", app_config.game_server_url);


}