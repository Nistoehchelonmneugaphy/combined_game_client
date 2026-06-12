use bevy::prelude::*;
use shared::AppState;


#[derive(Resource)]
pub struct Session {
    pub token: String,
    pub user_id: String,
}


#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::Launcher)]
pub enum LauncherState {
    #[default]
    Login,
    ServerSelect,
}
