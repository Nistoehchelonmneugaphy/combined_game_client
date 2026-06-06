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


/// Set title. On wasm32 it sets browser tab title, on PC it sets window title.
pub fn set_title(window: &mut Window, title: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        window.title = title.to_string();
    }
    #[cfg(target_arch = "wasm32")]
    set_browser_title(title);
}

#[cfg(target_arch = "wasm32")]
fn set_browser_title(title: &str) {
    let Some(window) = web_sys::window() else {return;};
    let Some(document) = window.document() else { return;};
    document.set_title(title);
}