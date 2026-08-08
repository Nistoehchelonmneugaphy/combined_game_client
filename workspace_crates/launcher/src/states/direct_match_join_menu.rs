use bevy::{
    prelude::*,
    text::{EditableText, TextCursorStyle},
};
use shared::{
    AppConfig, AppState, MatchPlayerToken, MatchServerAddress, MatchStateResource, set_title,
};

use crate::structs::LauncherState;


#[derive(Component)]
pub struct AddressInput;

#[derive(Component)]
pub struct AddressText;

#[derive(Component)]
pub struct AddressInputFieldCursor;

#[derive(Component)]
pub struct TokenInput;

#[derive(Component)]
pub struct TokenText;

#[derive(Component)]
pub struct TokenInputFieldCursor;

#[derive(Component)]
pub struct ConnectButton;

#[derive(Component)]
pub struct GoBackToLoginMenuButton;

pub fn setup_direct_match_join_menu_ui(mut commands: Commands, mut windows: Query<&mut Window>) {
    let launcher_title = "Directly Join A Match";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };

    // Root
    commands
        .spawn((
            DespawnOnExit(LauncherState::DirectMatchJoinMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
        ))
        .with_children(|p| {
            spawn_text_field_editable_text(
                p,
                "Address",
                AddressInput,
                AddressText,
                AddressInputFieldCursor,
                LauncherState::Login,
            );
            spawn_text_field_editable_text(
                p,
                "Token",
                TokenInput,
                TokenText,
                TokenInputFieldCursor,
                LauncherState::Login,
            );

            p.spawn((
                ConnectButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Connect!"));

            p.spawn((
                GoBackToLoginMenuButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(4.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("<- Go back to login menu"));
        });
}

pub fn add_launcher_direct_match_join_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_text_input,
            handle_connect_button,
            handle_back_to_login_menu_button,
        )
            .chain()
            .run_if(in_state(LauncherState::DirectMatchJoinMenu)),
    );
}

pub fn get_string_value<M, W>(query: &mut Query<(&mut EditableText, &M), Without<W>>) -> String
where
    M: Component,
    W: Component,
{
    {
        if let Ok((text_input, _)) = query.single_mut() {
            text_input.value().to_string()
        } else {
            "failed".to_string()
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn handle_connect_button(
    mut commands: Commands,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    config: Res<AppConfig>,
    mut next_state: ResMut<NextState<AppState>>,
    mut address_text: Query<(&mut EditableText, &AddressInput), Without<TokenInput>>,
    mut token_text: Query<(&mut EditableText, &TokenInput), Without<AddressInput>>,
) {
    for interaction in &interaction_q {
        let _config = config.clone();
        if *interaction != Interaction::Pressed {
            continue;
        }

        soft_keyboard_input::hide_soft_keyboard_if_needed();

        let address = get_string_value(&mut address_text);
        let token = get_string_value(&mut token_text);

        debug!("Address: {address}, token: {token}");
        commands.insert_resource(MatchStateResource::WantsToConnect {
            address: MatchServerAddress(address),
            player_token: MatchPlayerToken(token),
        });
        next_state.set(AppState::ConnectingToMatch);
    }
}

pub fn handle_back_to_login_menu_button(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<GoBackToLoginMenuButton>)>,
    mut next_state: ResMut<NextState<LauncherState>>,
) {
    for interaction in &interaction_q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        next_state.set(LauncherState::Login);
    }
}

pub fn handle_text_input(
    mut address_text: Query<(&mut EditableText, &AddressInput), Without<TokenInput>>,
    mut token_text: Query<(&mut EditableText, &TokenInput), Without<AddressInput>>,
    address_field_interaction_query: Query<&Interaction, With<AddressInput>>,
    token_field_interaction_query: Query<&Interaction, With<TokenInput>>,
    anywhere_else_interaction_query: Query<
        &Interaction,
        (Without<AddressInput>, Without<TokenInput>),
    >,
) {
    let address_pressed = address_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    let token_pressed = token_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    let anywhere_else_pressed = anywhere_else_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);

    if address_pressed {
        soft_keyboard_input::show_soft_keyboard_if_needed(&get_string_value(&mut address_text), "Address");
    } else if token_pressed {
        soft_keyboard_input::show_soft_keyboard_if_needed(&get_string_value(&mut token_text), "Token");
    } else if anywhere_else_pressed {
        soft_keyboard_input::hide_soft_keyboard_if_needed();
    }
}


// I think I will replace the old shared version with this one later on
pub fn spawn_text_field_editable_text<M: Bundle, T: Bundle, C: Bundle, S: States>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    container_marker: M,
    _text_marker: T,
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
                EditableText {
                    visible_width: Some(10.),
                    allow_newlines: false,
                    ..Default::default()
                },
                Node {
                    width: Val::Px(280.0),
                    height: Val::Px(36.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    ..default()
                },
                TextCursorStyle::default(),
                BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|row| {
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
