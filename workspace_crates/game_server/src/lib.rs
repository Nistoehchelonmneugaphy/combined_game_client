use bevy::prelude::*;
use shared::{AppConfig, AppState, Session, set_title};
pub struct GameServerPlugin;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::GameServer)]
pub enum GameServerState {
    #[default]
    Authentication,
    HomeScreen,
}

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameServerState>();
        app.add_systems(
            OnEnter(GameServerState::Authentication),
            (setup_authentication_ui, authenticate),
        );
       

        app.add_systems(OnEnter(GameServerState::HomeScreen), setup_homescreen_ui);

    }
}

fn setup_authentication_ui(mut commands: Commands, mut windows: Query<&mut Window>) {
    debug!("From setup_authentication_ui");
    let launcher_title = "Authenticating...";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut *window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };

    // Root
    commands
        .spawn((
            DespawnOnExit(GameServerState::Authentication),
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
            p.spawn((
                DespawnOnExit(GameServerState::Authentication),
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
            ))
            .with_child(Text::new("Authenticating to the server..."));
        });
}

fn setup_homescreen_ui() {}


fn authenticate(app_config: Res<AppConfig>, session: Res<Session>) {
    let app_config = app_config.clone();
    let session = session.clone();
    let game_server_url = app_config.game_server_url;
    // step 1: get nonce from server
    
    
}