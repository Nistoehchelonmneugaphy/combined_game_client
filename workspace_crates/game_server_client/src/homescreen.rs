use bevy::prelude::*;
use game_server_structs::websocket_communication::{
    ClientMessage, ClientMessageContent, ServerMessage,
};
use shared::set_title;

use crate::{GameServerState, ServerCookie, chat, websocket_connection::GameServerConnection};


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


pub fn setup_homescreen_ui(
    mut windows: Query<&mut Window>,
    cookie: Res<ServerCookie>,
    mut events: MessageWriter<ClientMessage>,
    mut commands: Commands,
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

    chat::spawn_chat_history_ui(&mut commands);
    queue_buttons(&mut commands);

}

fn queue_buttons(commands: &mut Commands) {
        commands
        .spawn((
            DespawnOnExit(GameServerState::HomeScreen),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
        ))
        .with_children(|p| {

            
            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Queue solo"));
            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Queue co-op"));
        });
}
