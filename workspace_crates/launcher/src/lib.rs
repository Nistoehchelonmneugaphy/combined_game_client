use bevy::prelude::*;

use crate::{
    states::{direct_match_join_menu::{add_launcher_direct_match_join_systems, setup_direct_match_join_menu_ui}, login::{
        add_launcher_login_systems, setup_launcher_login_ui, teardown_launcher_login_ui,
    }, server_select::{setup_server_select_ui, teardown_server_select_ui}}, structs::LauncherState,
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

        // Direct match join
        app.add_systems(OnEnter(LauncherState::DirectMatchJoinMenu), setup_direct_match_join_menu_ui);
        add_launcher_direct_match_join_systems(app);
    }
}
