use bevy::prelude::*;
use game_server_structs::websocket_communication::{
    ClientMessage, ClientMessageContent, ServerMessage,
};
use shared::set_title;
use tracing::instrument;

use crate::{GameServerState, ServerCookie, chat, websocket_connection::GameServerConnection};

#[derive(Resource, Clone)]
pub struct InQueue(pub bool);
#[derive(Component)]
pub struct EnterSeekerQueueButton;

#[derive(Component)]
pub struct EnterRunnerQueueButton;

#[derive(Component)]
pub struct ExitQueueButton;

#[derive(Message, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InternalMessage {
    SendChatMessage { message: String },
    EnterSoloQueue,
    EnterCoopQueue,
    ExitQueue,
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

pub fn process_internal_message(
    cookie: Res<ServerCookie>,
    mut event_reader: MessageReader<InternalMessage>,
    mut event_writer: MessageWriter<ClientMessage>,
) {
    let cookie = match cookie.0.clone() {
        Some(c) => c,
        None => {
            error!("Cookie is none");
            return;
        }
    };
    for msg in event_reader.read() {
        match msg {
            InternalMessage::SendChatMessage { message } => {
                debug!("Send msg");
                event_writer.write(ClientMessage {
                    cookie: cookie.clone(),
                    content: ClientMessageContent::SendGlobalChatMessage {
                        content: message.to_string(),
                    },
                });
            }   
            InternalMessage::EnterSoloQueue => {
                debug!("InternalMessage::EnterSoloQueue");
            }
            InternalMessage::EnterCoopQueue => {
                debug!("InternalMessage::EnterCoopQueue");
            }
            InternalMessage::ExitQueue => {
                  debug!("InternalMessage::ExitQueue");
            }
        }
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
    debug!("Calling queue_buttons");
    spawn_queue_buttons(&mut commands);
}

#[instrument(skip_all)]
fn spawn_queue_buttons(commands: &mut Commands) {
    debug!("Spawning queue buttons");
    commands.insert_resource(InQueue(false));
    commands
        .spawn((
            DespawnOnExit(GameServerState::HomeScreen),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                EnterSeekerQueueButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Queue as hunter"));

            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                EnterRunnerQueueButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Queue as survivor"));

            p.spawn((
                DespawnOnExit(GameServerState::HomeScreen),
                Visibility::Hidden,
                ExitQueueButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Exit queue"));
        });
}

pub fn queue_buttons_handler(
    mut event_writer: MessageWriter<InternalMessage>,
    solo_interaction_q: Query<&Interaction, (Changed<Interaction>, With<EnterSeekerQueueButton>)>,
    coop_interaction_q: Query<&Interaction, (Changed<Interaction>, With<EnterRunnerQueueButton>)>,
    exit_interaction_q: Query<&Interaction, (Changed<Interaction>, With<ExitQueueButton>)>,
    mut exit_button_q: Query<
        &mut Visibility,
        (
            With<ExitQueueButton>,
            Without<EnterSeekerQueueButton>,
            Without<EnterRunnerQueueButton>,
        ),
    >,
    mut solo_button_q: Query<
        &mut Visibility,
        (
            With<EnterSeekerQueueButton>,
            Without<ExitQueueButton>,
            Without<EnterRunnerQueueButton>,
        ),
    >,
    mut coop_button_q: Query<
        &mut Visibility,
        (
            With<EnterRunnerQueueButton>,
            Without<ExitQueueButton>,
            Without<EnterSeekerQueueButton>,
        ),
    >,
    mut in_queue: ResMut<InQueue>,
) {
    if in_queue.0 {
        for interaction in &exit_interaction_q {
            if *interaction != Interaction::Pressed {
                continue;
            }
            debug!("Exiting queue");
            event_writer.write(InternalMessage::ExitQueue);
            *in_queue = InQueue(false);
            set_buttons_visibility_based_on_if_in_queue(
                in_queue.0,
                &mut exit_button_q,
                &mut solo_button_q,
                &mut coop_button_q,
            );

            return;
        }
    } else {
        for interaction in &solo_interaction_q {
            if *interaction != Interaction::Pressed {
                continue;
            }
            debug!("Entering solo queue");
            event_writer.write(InternalMessage::EnterSoloQueue);
            *in_queue = InQueue(true);
            set_buttons_visibility_based_on_if_in_queue(
                in_queue.0,
                &mut exit_button_q,
                &mut solo_button_q,
                &mut coop_button_q,
            );
            return;
        }
        for interaction in &coop_interaction_q {
            if *interaction != Interaction::Pressed {
                continue;
            }
            debug!("Entering coop queue");
            event_writer.write(InternalMessage::EnterCoopQueue);
            *in_queue = InQueue(true);
            set_buttons_visibility_based_on_if_in_queue(
                in_queue.0,
                &mut exit_button_q,
                &mut solo_button_q,
                &mut coop_button_q,
            );

            return;
        }
    }
}

fn set_buttons_visibility_based_on_if_in_queue(
    in_queue: bool,
    exit_button_q: &mut Query<
        &mut Visibility,
        (
            With<ExitQueueButton>,
            Without<EnterSeekerQueueButton>,
            Without<EnterRunnerQueueButton>,
        ),
    >,
    solo_button_q: &mut Query<
        &mut Visibility,
        (
            With<EnterSeekerQueueButton>,
            Without<ExitQueueButton>,
            Without<EnterRunnerQueueButton>,
        ),
    >,
    coop_button_q: &mut Query<
        &mut Visibility,
        (
            With<EnterRunnerQueueButton>,
            Without<ExitQueueButton>,
            Without<EnterSeekerQueueButton>,
        ),
    >,
) {
    let exit_button_visibility = if in_queue {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let enter_queue_buttons_visibility = if in_queue {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    for mut vis in exit_button_q {
        *vis = exit_button_visibility.clone();
    }
    for mut vis in solo_button_q {
        *vis = enter_queue_buttons_visibility.clone();
    }
    for mut vis in coop_button_q {
        *vis = enter_queue_buttons_visibility.clone();
    }
}
