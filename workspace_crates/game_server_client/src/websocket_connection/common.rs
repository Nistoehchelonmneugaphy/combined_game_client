use game_server_structs::websocket_communication::{ClientMessage, ServerMessage};
use tokio::sync::mpsc;
use bevy::prelude::*;


/// Platform-agnostic view of a WS message, so parsing logic is written once.
pub(crate) enum WsMessage {
    Text(String),
    Close,
    Other, // binary/ping/pong/etc — ignored uniformly
}

/// Shared serialization logic for outgoing messages.
pub(crate) fn serialize_outgoing(msg: &ClientMessage) -> Option<String> {
    match serde_json::to_string(msg) {
        Ok(text) => Some(text),
        Err(e) => {
            warn!("Failed to serialize ClientMessage: {e}");
            None
        }
    }
}

/// Shared parsing logic — used by both native and wasm backends.
pub(crate) fn handle_incoming(
    from_server_tx: &mpsc::UnboundedSender<ServerMessage>,
    msg: WsMessage,
) -> BreakOrContinue {
    match msg {
        WsMessage::Text(text) => {
            match serde_json::from_str::<ServerMessage>(&text) {
                Ok(parsed) => {
                    if from_server_tx.send(parsed).is_err() {
                        warn!("from_server channel closed, receiver dropped");
                        return BreakOrContinue::Break;
                    }
                }
                Err(e) => warn!("Failed to parse ServerMessage: {e}, raw: {text}"),
            }
        }
        WsMessage::Close => {
            warn!("Server closed connection");
            return BreakOrContinue::Break;
        }
        WsMessage::Other => {
            trace!("Ignoring non-text WS message");
        }
    }
    BreakOrContinue::Continue
}

pub enum BreakOrContinue {
    Break,
    Continue,
}