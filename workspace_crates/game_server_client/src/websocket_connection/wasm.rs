use bevy::prelude::*;
// ws_wasm.rs
use crate::websocket_connection::common::{
    BreakOrContinue, WsMessage, handle_incoming, serialize_outgoing,
};
use futures_util::{SinkExt, StreamExt};
use game_server_public_api::websocket_communication::{ClientMessage, ServerMessage};
use gloo_net::websocket::{Message, futures::WebSocket};
use tokio::sync::mpsc; // tokio's mpsc works fine on wasm32 (it's just a lock-free queue, no OS threads needed)

pub fn spawn_ws_task(
    url: String,
    cookie: String,
    to_server_rx: mpsc::UnboundedReceiver<ClientMessage>,
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
) {
    wasm_bindgen_futures::spawn_local(ws_loop(url, cookie, to_server_rx, from_server_tx));
}

async fn ws_loop(
    url: String,
    cookie: String,
    mut to_server_rx: mpsc::UnboundedReceiver<ClientMessage>,
    from_server_tx: mpsc::UnboundedSender<ServerMessage>,
) {
    let encoded_cookie = game_server_public_api::websocket_communication::pack_cookie(cookie);

    // apparently, sec-websocket-protocol is the only header that browsers allow websockets to send?
    // so, unlike native, I have  to either 1) send token over query string along with url or 2) send it after establishing ws connection (don't really feel like it) or 3) use the sec-websocket-protocol header, which normally has a different purpose.
    let protocols: &[&str] = &[encoded_cookie.as_str()];
    let ws = match WebSocket::open_with_protocols(&url, protocols) {
        Ok(ws) => ws,
        Err(e) => {
            warn!("Failed to open WebSocket: {e:#?}");
            return;
        }
    };
    let (mut writer, mut reader) = ws.split();

    loop {
        tokio::select! {
            msg = reader.next() => {
                let ws_msg = to_ws_message(msg);
                match handle_incoming(&from_server_tx, ws_msg) {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
            Some(msg) = to_server_rx.recv() => {
                match send_one(&mut writer, &msg).await {
                    BreakOrContinue::Break => break,
                    BreakOrContinue::Continue => {}
                }
            }
        }
    }
}

fn to_ws_message(msg: Option<Result<Message, gloo_net::websocket::WebSocketError>>) -> WsMessage {
    match msg {
        Some(Ok(Message::Text(text))) => WsMessage::Text(text),
        Some(Ok(Message::Bytes(_))) => WsMessage::Other,
        Some(Err(e)) => {
            warn!("WS read error: {e:?}");
            WsMessage::Close
        }
        None => WsMessage::Close,
    }
}

async fn send_one(
    writer: &mut (impl futures_util::Sink<Message, Error = gloo_net::websocket::WebSocketError> + Unpin),
    msg: &ClientMessage,
) -> BreakOrContinue {
    let Some(text) = serialize_outgoing(msg) else {
        return BreakOrContinue::Continue;
    };
    match writer.send(Message::Text(text)).await {
        Ok(()) => BreakOrContinue::Continue,
        Err(e) => {
            warn!("WS write failed: {e:?}");
            BreakOrContinue::Break
        }
    }
}
