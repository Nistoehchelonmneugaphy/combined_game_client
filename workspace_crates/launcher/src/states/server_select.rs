use bevy::prelude::*;
use shared::{AppConfig, AppState};

pub fn teardown_server_select_ui() {

}

pub fn setup_server_select_ui(app_config: Res<AppConfig>,     mut next_state: ResMut<NextState<AppState>>,) {
    debug!("From setup_server_select_ui");
    let _app_config = app_config.clone();
    // TODO maybe add proper server seleciton later, if I wanted
    // For now, just immediately go into AppState::Server
    debug!("Switching to AppState::GameServer");
     next_state.set(AppState::GameServer);

}