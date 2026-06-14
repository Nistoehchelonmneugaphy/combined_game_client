use bevy::prelude::*;

use crate::{
    states::login::{
        add_launcher_login_systems, setup_launcher_login_ui, teardown_launcher_login_ui,
    },
    states::server_select::{setup_server_select_ui, teardown_server_select_ui},
    structs::LauncherState,
};
mod states;
mod structs;
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LauncherState>();
        app.add_systems(OnEnter(LauncherState::Login), setup_launcher_login_ui);
        app.add_systems(OnExit(LauncherState::Login), teardown_launcher_login_ui);
        add_launcher_login_systems(app);

        app.add_systems(OnEnter(LauncherState::ServerSelect), setup_server_select_ui);
        app.add_systems(
            OnExit(LauncherState::ServerSelect),
            teardown_server_select_ui,
        );
    }
}
