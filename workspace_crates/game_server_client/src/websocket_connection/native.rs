use bevy::prelude::*;
// ws_native.rs
use crate::websocket_connection::common::{
    BreakOrContinue, WsMessage, handle_incoming, serialize_outgoing,
};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use game_server_structs::websocket_communication::{ClientMessage, ServerMessage};
use http::HeaderValue;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::client::IntoClientRequest,
    tungstenite::{self, Message},
};

pub fn spawn_ws_task(
    url: String,
    cookie: String,
    to_server_rx: mpsc::UnboundedReceiver<ClientMessage>,
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to start Tokio runtime");
        rt.block_on(ws_loop(url, cookie, to_server_rx, from_server_tx));
    });
}

async fn ws_loop(
    url: String,
    cookie: String,
    mut to_server_rx: mpsc::UnboundedReceiver<ClientMessage>,
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
) {
    let mut request = match url.clone().into_client_request() {
        Ok(r) => r,
        Err(e) => {
            warn!("Invalid WS URL {url}: {e}");
            return;
        }
    };
    let cookie_value = match HeaderValue::from_str(&cookie) {
        Ok(v) => v,
        Err(e) => {
            warn!("Invalid cookie header value: {e}");
            return;
        }
    };
    request.headers_mut().insert("cookie", cookie_value);

    let (ws_stream, _) = match tokio_tungstenite::connect_async(request).await {
        Ok(pair) => pair,
        Err(e) => {
            warn!("Failed to connect to ws server: {e}");
            return;
        }
    };
    let (mut ws_writer, mut ws_reader) = ws_stream.split();

    loop {
        tokio::select! {
            msg = ws_reader.next() => {
                let ws_msg = to_ws_message(msg);
                match handle_incoming(&from_server_tx, ws_msg) {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
            Some(msg) = to_server_rx.recv() => {
                match send_one(&mut ws_writer, &msg).await {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
        }
    }
}

fn to_ws_message(
    msg: Option<Result<tungstenite::protocol::Message, tungstenite::error::Error>>,
) -> WsMessage {
    match msg {
        Some(Ok(Message::Text(text))) => WsMessage::Text(text.to_string()),
        Some(Ok(Message::Close(_))) | None => WsMessage::Close,
        Some(Ok(_)) => WsMessage::Other,
        Some(Err(e)) => {
            warn!("WS read error: {e}");
            WsMessage::Close
        }
    }
}

async fn send_one(
    ws_writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    msg: &ClientMessage,
) -> BreakOrContinue {
    let Some(text) = serialize_outgoing(msg) else {
        return BreakOrContinue::Continue;
    };
    match ws_writer.send(Message::Text(text.into())).await {
        Ok(()) => BreakOrContinue::Continue,
        Err(e) => {
            warn!("WS write failed: {e}");
            BreakOrContinue::Break
        }
    }
}
