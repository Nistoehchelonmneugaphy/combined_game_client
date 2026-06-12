
use bevy::prelude::*;

use crate::{login::{add_launcher_login_systems, setup_launcher_login_ui, teardown_launcher_login_ui}, server_select::{setup_server_select_ui, teardown_server_select_ui}, structs::LauncherState};
mod structs;
mod server_select;
mod login;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LauncherState>();
        app.add_systems(OnEnter(LauncherState::Login), setup_launcher_login_ui);
        app.add_systems(OnExit(LauncherState::Login), teardown_launcher_login_ui);
       add_launcher_login_systems(app);

        app.add_systems(OnEnter(LauncherState::ServerSelect), setup_server_select_ui);
        app.add_systems(OnExit(LauncherState::ServerSelect), teardown_server_select_ui);


    }
}
