
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
    pub ws_game_server_url: String,
    pub directly_join_a_match_on_startup: bool,
    pub direct_match_address: String,
    pub direct_match_token: String,
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
    pub ws_game_server_url: String,
    pub directly_join_a_match_on_startup: bool,
    pub direct_match_address: String,
    pub direct_match_token: String,
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



pub fn spawn_text_field<M: Bundle, T: Bundle, C: Bundle, S: States>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    container_marker: M,
    text_marker: T,
    cursor_marker: C,
    state_despawn: S,
) {
    parent
        .spawn((
            DespawnOnExit(state_despawn),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            },
        ))
        .with_children(|col| {
            col.spawn(Text::new(label));

            col.spawn((
                container_marker,
                Button,
                Node {
                    width: Val::Px(280.0),
                    height: Val::Px(36.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|row| {
                row.spawn((text_marker, Text::new("")));
                // Cursor: thin rectangle
                row.spawn((
                    cursor_marker,
                    Node {
                        width: Val::Px(2.0),
                        height: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    Visibility::Hidden, // starts hidden
                ));
            });
        });
}

#[derive(Resource)]
pub struct CursorBlink(pub Timer);
impl Default for CursorBlink {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}
