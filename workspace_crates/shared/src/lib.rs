use bevy::prelude::*;
use serde::Deserialize;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Startup,
    Launcher,
    Server,
    Match,
}

#[derive(Resource, Clone, Debug)]
pub struct AppConfig {
    pub auth_server_url: String,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct AppConfigAsset {
    pub auth_server_url: String,
}

#[derive(Resource)]
pub struct AppConfigHandle(pub Handle<AppConfigAsset>);