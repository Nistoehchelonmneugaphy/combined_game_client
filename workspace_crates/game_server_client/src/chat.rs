use std::collections::VecDeque;

use bevy::prelude::*;
use game_server_structs::websocket_communication::ServerMessage;

use crate::GameServerState;

#[derive(Component)]
pub struct ChatMessageSlot(pub usize); // 0 = oldest, 4 = newest

#[derive(Resource, Default)]
pub struct ChatHistory {
    pub messages: VecDeque<String>, // max 5
}



pub fn spawn_chat_history_ui(mut commands: Commands) {
    commands.insert_resource(ChatHistory::default());
    // Spawn a root node, then 5 text slots as children
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
        .with_children(|parent| {
            for i in 0..5 {
                parent.spawn((
                    Text::new("msg will be here"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Visibility::Visible, // invisible until it has content
                    ChatMessageSlot(i),
                ));
            }
        });

       
}

pub fn receive_chat_messages(
    mut server_events: MessageReader<ServerMessage>,
    mut history: ResMut<ChatHistory>,
    mut slots: Query<(&ChatMessageSlot, &mut Text, &mut Visibility)>,
) {
    let mut got_new = false;

    for msg in server_events.read() {
        if let ServerMessage::NewGlobalChatMessage {
            content,
            username,
            user_id,
        } = &msg
        {
            history
                .messages
                .push_back(format!("{}: {}", username, content));
            if history.messages.len() > 5 {
                history.messages.pop_front(); // shift out oldest
            }
            got_new = true;
        }
    }

    if !got_new {
        return;
    }

    // Re-render all slots from current history
    // Pad from the front so newest message is always at slot 4
    for (slot, mut text, mut visibility) in &mut slots {
        let history_index = slot.0 + 5 - history.messages.len(); // may underflow
        if slot.0 + 5 < history.messages.len() || history_index >= 5 {
            // slot is before our oldest message — hide it
            *visibility = Visibility::Hidden;
            text.0 = "".to_string();
        } else if let Some(msg) = history
            .messages
            .get(slot.0.wrapping_sub(5 - history.messages.len()))
        {
            text.0 = msg.clone();
            *visibility = Visibility::Inherited;
        }
    }
}
