use bevy::prelude::*;
use game_server_public_api::websocket_communication::{ClientMessage, ServerMessage};
use shared::AppConfig;
use tokio::sync::mpsc;

use crate::ServerCookie;

mod common;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use native::spawn_ws_task;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::spawn_ws_task;

#[derive(Resource)]
pub struct GameServerConnection {
    pub to_server: mpsc::UnboundedSender<ClientMessage>,
    pub from_server: mpsc::UnboundedReceiver<ServerMessage>,
}

pub fn setup_connection(
    app_config: Res<AppConfig>,
    mut commands: Commands,
    cookie: Res<ServerCookie>,
) {
    let (to_server_tx, to_server_rx) = mpsc::unbounded_channel::<ClientMessage>();
    let (from_server_tx, from_server_rx) = mpsc::unbounded_channel::<ServerMessage>();
    let app_config = app_config.clone();
    let cookie = cookie.clone();
    let Some(cookie) = cookie.0 else {
        debug!("No cookie");
        return;
    };
    spawn_ws_task(
        app_config.ws_game_server_url.clone(),
        cookie,
        to_server_rx,
        from_server_tx,
    );

    commands.insert_resource(GameServerConnection {
        to_server: to_server_tx,
        from_server: from_server_rx,
    });
}
