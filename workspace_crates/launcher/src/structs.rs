use bevy::prelude::*;
use shared::AppState;



#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::Launcher)]
pub enum LauncherState {
    #[default]
    Login,
    ServerSelect,
    DirectMatchJoinMenu,
}
