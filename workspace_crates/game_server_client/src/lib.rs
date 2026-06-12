use bevy::prelude::*;
use bevy_mod_reqwest::{BevyReqwest, ReqwestErrorEvent, ReqwestPlugin, ReqwestResponseEvent};
use game_server_structs::{ServerNonceResponse, get_server_path};
use shared::{AppConfig, AppState, Session, set_title};
pub struct GameServerClientPlugin;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::GameServer)]
pub enum GameServerState {
    #[default]
    Authentication,
    HomeScreen,
}

impl Plugin for GameServerClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameServerState>();
        app.add_plugins(ReqwestPlugin::default());
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

fn authenticate(app_config: Res<AppConfig>, session: Res<Session>, mut client: BevyReqwest) {
    debug!("Entered authenticate");
    let app_config = app_config.clone();
    let _session = session.clone();
    let game_server_url = app_config.game_server_url;
    // step 1: get nonce from server
    let url = get_server_path(
        &game_server_url,
        game_server_structs::ServerAction::InitLogin,
    );
    let req = client.post(url).build().unwrap();
    client
        .send(req)
          .on_response(|req: On<ReqwestResponseEvent>| {
            let req = req.event();
            let Ok(nonce) = serde_json::from_slice::<ServerNonceResponse>(&req.body()) else {
                    error!("Could not shape response into ServerNonceResponse.");
                    error!("Response bytes: {:#?}, response headers: {:#?}",String::from_utf8_lossy(&req.body()), &req.response_headers());
                    return;
                };
            debug!("Step 1 success: nonce: {nonce:?}");
        }) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
