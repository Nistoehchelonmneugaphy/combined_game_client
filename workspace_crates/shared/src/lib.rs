use bevy::prelude::*;
use serde::Deserialize;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Startup,
    Launcher,
    GameServer,
    Match,
}

#[derive(Resource, Clone, Debug)]
pub struct AppConfig {
    pub auth_server_url: String,
    pub game_server_url: String,
}

#[derive(Resource, Clone, Debug)]
pub struct Session {
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct AppConfigAsset {
    pub auth_server_url: String,
    pub game_server_url: String,
}

#[derive(Resource)]
pub struct AppConfigHandle(pub Handle<AppConfigAsset>);


/// Set title. On wasm32 it sets browser tab title, on PC it sets window title. On Android/IOS it does nothing, I don't think either have any window title in any form, right?
pub fn set_title(window: &mut Window, title: &str) {
    set_title_impl(window, title);
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
fn set_title_impl(window: &mut Window, title: &str) {
    debug!("Setting window title to {title}");
    window.title = title.to_string();
}

#[cfg(target_arch = "wasm32")]
pub fn set_title_impl(_window: &mut Window, title: &str) {
    set_browser_title(title);
}

#[cfg(target_arch = "wasm32")]
fn set_browser_title(title: &str) {
    debug!("Setting browser title to {title}");
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    document.set_title(title);
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn set_title_impl(_window: &mut Window, _title: &str) {}
