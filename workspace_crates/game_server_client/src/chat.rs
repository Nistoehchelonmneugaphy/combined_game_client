use std::collections::VecDeque;

use bevy::prelude::*;
use game_server_structs::websocket_communication::{
    ClientMessage, ClientMessageContent, ServerMessage,
};
use shared::spawn_text_field;

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_text_input, handle_send_chat_message_button)
                .chain()
                .run_if(in_state(GameServerState::HomeScreen)),
        );
    }
}
use crate::{GameServerState, ServerCookie};

#[derive(Resource, Default)]
pub struct ChatHistory {
    pub _messages: VecDeque<String>,
}
#[derive(Component)]
pub struct ChatMessageText;
#[derive(Component)]
pub struct ChatMessageInput;
#[derive(Component)]
pub struct ChatMessageInputFieldCursor;
#[derive(Component)]
pub struct SendChatMessageButton;

#[derive(Resource, Default)]
pub struct ChatMessageForm {
    pub chat_message: String,
    pub focused: ChatMessageFormField,
}
#[derive(Default, PartialEq)]
pub enum ChatMessageFormField {
    #[default]
    ChatMessage,
}

pub fn spawn_chat_history_ui(mut commands: Commands) {
    commands.init_resource::<ChatMessageForm>();
    commands.insert_resource(ChatHistory::default());
    commands.insert_resource(CurrentChatMessageSlot(0));
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
        .with_children(|p| {
            for i in 0..=5 {
                p.spawn((
                    Text::new("msg will be here"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Visibility::Visible,
                    ChatMessageSlot(i),
                ));
            }
            spawn_text_field(
                p,
                "Chat message...",
                ChatMessageInput,
                ChatMessageText,
                ChatMessageInputFieldCursor,
                GameServerState::HomeScreen,
            );

            // Login button
            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                SendChatMessageButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Send!"));
        });
}

pub const MAX_CHAT_SLOT_INDEX: usize = 5;

#[derive(Component)]
pub struct ChatMessageSlot(pub usize);

#[derive(Resource)]
pub struct CurrentChatMessageSlot(pub usize);

pub fn receive_chat_messages(
    mut server_events: MessageReader<ServerMessage>,
    mut _history: ResMut<ChatHistory>,
    mut slots: Query<(&ChatMessageSlot, &mut Text, &mut Visibility)>,
    mut current_slot_index: ResMut<CurrentChatMessageSlot>,
) {
    let mut repopulate = false;
    // copy all current slot's text into vec,
    let mut slot_contents: Vec<String> = Vec::new();
    for (slot, text, _) in &slots {
        slot_contents[slot.0] = text.0.clone();
    }

    for msg in server_events.read() {
        let ServerMessage::NewGlobalChatMessage {
            content,
            username,
            user_id: _,
        } = &msg
        else {
            continue;
        };

        let formatted_msg = format!("{username}: {content}");
        debug!("Received message: {formatted_msg}");

        if current_slot_index.0 == MAX_CHAT_SLOT_INDEX {
            repopulate = true;

            // make new vec where prev vec's element 0 is removed. new message is at last index, and second message is now first (at index 0)
            let mut new_vec: Vec<String> = slot_contents.clone();
            new_vec.remove(0);
            new_vec[current_slot_index.0] = formatted_msg.clone();
            slot_contents = new_vec.clone();
        }
        if current_slot_index.0 < MAX_CHAT_SLOT_INDEX {
            for (slot, mut text, mut visibility) in &mut slots {
                if slot.0 == current_slot_index.0 {
                    text.0 = format!("{username}: {content}");
                    *visibility = Visibility::Inherited;
                    *current_slot_index = CurrentChatMessageSlot(current_slot_index.0 + 1);
                    break;
                }
            }
        }
    }
    if repopulate {
        for (slot, mut text, mut visibility) in &mut slots {
            for (index, c) in slot_contents.iter().enumerate() {
                if slot.0 == index {
                    text.0 = c.clone();
                    *visibility = Visibility::Inherited;
                    *current_slot_index = CurrentChatMessageSlot(current_slot_index.0 + 1);
                    break;
                }
            }
        }
    }
}

pub fn handle_send_chat_message_button(
    mut form: ResMut<ChatMessageForm>,
    cookie: Res<ServerCookie>,
    mut events: MessageWriter<ClientMessage>,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<SendChatMessageButton>)>,
    mut chat_message_text: Query<&mut Text, With<ChatMessageText>>,
) {
    for interaction in &interaction_q {
        if *interaction != Interaction::Pressed {
            continue;
        }

        #[cfg(target_arch = "wasm32")]
        {
            if web::is_mobile() && web_mobile_input::is_keyboard_shown() {
                web_mobile_input::hide_keyboard();
            }
        }

        let message = form.chat_message.clone();
        if let Some(cookie) = cookie.clone().0 {
            debug!("Send msg");
            events.write(ClientMessage {
                cookie,
                content: ClientMessageContent::SendGlobalChatMessage { content: message },
            });
            form.chat_message = "".to_string();
            if let Ok(mut text) = chat_message_text.single_mut() {
                text.0 = form.chat_message.clone();
            }
        }
    }
}

pub fn handle_text_input(
    mut form: ResMut<ChatMessageForm>,
    mut evr: MessageReader<bevy::input::keyboard::KeyboardInput>,
    chat_message_field_interaction_query: Query<&Interaction, With<ChatMessageInput>>,
    #[cfg(target_os = "android")] anywhere_else_interaction_query: Query<
        &Interaction,
        Without<ChatMessageInput>,
    >,
    mut chat_message_text: Query<&mut Text, With<ChatMessageText>>,
) {
    let chat_message_pressed = chat_message_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    #[cfg(target_os = "android")]
    let anywhere_else_pressed = anywhere_else_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);

    if chat_message_pressed {
        form.focused = ChatMessageFormField::ChatMessage;
    }
    #[cfg(target_arch = "wasm32")]
    {
        if web::is_mobile() && !web_mobile_input::is_keyboard_shown() && (chat_message_pressed) {
            web_mobile_input::show_keyboard();
        }
    }
    #[cfg(target_os = "android")]
    {
        if chat_message_pressed {
            android_mobile_input::show_keyboard(&form.chat_message, "Chat message");
        } else if anywhere_else_pressed {
            android_mobile_input::hide_keyboard();
        }
    }
    for ev in evr.read() {
        if ev.state != bevy::input::ButtonState::Pressed {
            continue;
        }
        if ev.logical_key == bevy::input::keyboard::Key::Backspace {
            let target = match form.focused {
                ChatMessageFormField::ChatMessage => &mut form.chat_message,
            };
            target.pop();
            continue;
        }

        // do not want to put "enter" into the text fields
        if ev.logical_key == bevy::input::keyboard::Key::Enter {
            continue;
        }

        // Use the `text` field for printable characters
        if let Some(ref s) = ev.text {
            let target = match form.focused {
                ChatMessageFormField::ChatMessage => &mut form.chat_message,
            };
            target.push_str(s);
        }
    }

    // Sync to UI - now these queries will actually find the entities
    if let Ok(mut text) = chat_message_text.single_mut() {
        text.0 = form.chat_message.clone();
    }
}
