use bevy::prelude::*;
use shared::AppState;

use crate::structs::{
    CursorBlink, ErrorLabel, LoginButton, LoginForm, LoginResultQueue, PasswordInput, PasswordInputFieldCursor, PasswordText, UsernameInput, UsernameInputFieldCursor, UsernameText
};
mod structs;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Launcher), setup_launcher_ui);
        app.add_systems(OnExit(AppState::Launcher), teardown_launcher_ui);
    }
}

fn setup_launcher_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(AppState::Launcher),
        AudioPlayer::new(asset_server.load("audio/bemcupa_petos.mp3")),
        PlaybackSettings::LOOP,
    ));
    commands.spawn((DespawnOnExit(AppState::Launcher), Camera2d));
    commands.init_resource::<LoginForm>();
    commands.init_resource::<CursorBlink>();
    commands.init_resource::<LoginResultQueue>();


    // Root
    commands
        .spawn((
            DespawnOnExit(AppState::Launcher),
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
            spawn_text_field(
                p,
                "Username",
                UsernameInput,
                UsernameText,
                UsernameInputFieldCursor,
            );
            spawn_text_field(
                p,
                "Password",
                PasswordInput,
                PasswordText,
                PasswordInputFieldCursor,
            );

            // Login button
            p.spawn((
                DespawnOnExit(AppState::Launcher),
                LoginButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Login"));

            // Error label (hidden until needed)
            p.spawn((
                DespawnOnExit(AppState::Launcher),
                ErrorLabel,
                Text::new(""),
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
        });
}

fn spawn_text_field<M: Bundle, T: Bundle, C: Bundle>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    container_marker: M,
    text_marker: T,
    cursor_marker: C,
) {
    parent
        .spawn((
            DespawnOnExit(AppState::Launcher),
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
                Button,
                Node {
                    width: Val::Px(280.0),
                    height: Val::Px(36.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|row| {
                row.spawn((text_marker, Text::new("")));
                // Cursor: thin rectangle
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

fn teardown_launcher_ui(mut commands: Commands) {
    // I assume that the DespawnOnExit takes care of all spawn'ed entities.
    // So, I guess I only need to do something like this... Remove the LoginForm? Since DespawnOnExit only works on commands.spawn. I think?..
    commands.remove_resource::<LoginForm>();
    commands.remove_resource::<CursorBlink>();
    commands.remove_resource::<LoginResultQueue>();
}
