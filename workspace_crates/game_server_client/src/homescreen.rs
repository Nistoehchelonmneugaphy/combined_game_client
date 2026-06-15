use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use game_server_structs::websocket_communication::{
    ClientMessage, ClientMessageContent, ServerMessage,
};
use shared::{AppConfig, set_title};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{self, Message},
};

use crate::ServerCookie;

#[derive(Resource)]
pub struct GameServerConnection {
    to_server: mpsc::UnboundedSender<ClientMessage>,
    from_server: mpsc::UnboundedReceiver<ServerMessage>,
}

pub fn setup_connection(app_config: Res<AppConfig>, mut commands: Commands) {
    let (to_server_tx, to_server_rx) = mpsc::unbounded_channel::<ClientMessage>();
    let (from_server_tx, from_server_rx) = mpsc::unbounded_channel::<ServerMessage>();
    let app_config = app_config.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to start Tokio runtime");
        rt.block_on(async move {
            ws_thread(
                app_config.clone().ws_game_server_url,
                to_server_rx,
                from_server_tx,
            )
            .await;
        });
    });
    commands.insert_resource(GameServerConnection {
        to_server: to_server_tx,
        from_server: from_server_rx,
    });
}
async fn ws_thread(
    url: String,
    mut to_server_rx: mpsc::UnboundedReceiver<ClientMessage>,
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
) {
    let (ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .expect("Failed to connect to ws server");
    let (mut ws_writer, mut ws_reader) = ws_stream.split();
    loop {
        tokio::select! {
            msg = ws_reader.next() => receive_ws_message(from_server_tx.clone(), msg).await,
            Some(msg) = to_server_rx.recv() => send_ws_message(&mut ws_writer, msg).await,
        }
    }
}
async fn receive_ws_message(
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
    msg: Option<Result<tungstenite::protocol::Message, tungstenite::error::Error>>,
) {
    match msg {
        Some(Ok(tungstenite::protocol::Message::Text(text))) => {
            if let Ok(parsed) = serde_json::from_str::<ServerMessage>(&text) {
                from_server_tx.send(parsed).unwrap();
            }
        }
        Some(Ok(tungstenite::protocol::Message::Close(_))) | None => {
            warn!("Server closed connection");
            return;
        }
        _ => {}
    }
}
async fn send_ws_message(
    ws_writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    msg: ClientMessage,
) {
    let text = serde_json::to_string(&msg).unwrap();
    ws_writer.send(Message::Text(text.into())).await.unwrap();
}
pub fn setup_homescreen_ui(
    mut windows: Query<&mut Window>,
    cookie: Res<ServerCookie>,
    mut events: MessageWriter<ClientMessage>,
) {
    debug!("Entering setup_homescreen_ui");
    if let Some(cookie) = cookie.clone().0 {
        debug!("Send test msg");
        events.write(ClientMessage {
            cookie,
            content: ClientMessageContent::SendGlobalChatMessage {
                content: "Hey bro and sis".to_string(),
            },
        });
    }
    let cookie = cookie.clone();
    debug!("Server cookie: {:#?} ", cookie.0);

    let launcher_title = "Homescreen";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut *window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };
}

pub fn receive_from_server(
    mut connection: ResMut<GameServerConnection>,
    mut events: MessageWriter<ServerMessage>,
) {
    // try_recv is non-blocking — if nothing there, just returns
    while let Ok(msg) = connection.from_server.try_recv() {
        events.write(msg);
    }
}

// any bevy system can send by writing a ToServer event
pub fn send_to_server(
    connection: Res<GameServerConnection>,
    mut events: MessageReader<ClientMessage>,
) {
    for msg in events.read() {
        connection.to_server.send(msg.clone()).ok();
    }
}
