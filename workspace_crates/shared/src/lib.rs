use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Launcher,
    Server,
    Match,
}

#[derive(Resource, Clone, Debug)]
pub struct AppConfig {
    pub auth_server_url: String,
}