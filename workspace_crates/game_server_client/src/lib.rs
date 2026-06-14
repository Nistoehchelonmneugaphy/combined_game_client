use auth_system_structs::public_api::{CreateJoinTokenRequest, CreateJoinTokenResponse};
use bevy::prelude::*;
use bevy_mod_reqwest::{BevyReqwest, JsonResponse, ReqwestErrorEvent, ReqwestPlugin};
use game_server_structs::{
    FinalLoginStepRequest, FinalLoginStepResponse, ServerNonceResponse, get_server_path,
};
use shared::{AppConfig, AppState, Session, set_title};
pub struct GameServerClientPlugin;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::GameServer)]
pub enum GameServerState {
    #[default]
    Authentication,
    HomeScreen,
}

#[derive(Resource, Debug, Clone)]
struct ServerCookie(Option<String>);

impl Plugin for GameServerClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameServerState>();
        app.add_plugins(ReqwestPlugin::default());
        app.add_systems(
            OnEnter(GameServerState::Authentication),
            (authenticate, setup_authentication_ui),
        );
        app.insert_resource(ServerCookie(None));
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

fn setup_homescreen_ui(mut windows: Query<&mut Window>, cookie: Res<ServerCookie>) {
    debug!("Entering setup_homescreen_ui");

    let cookie = cookie.clone();
    debug!("Server cookie: {:#?} ", cookie.0);

    let launcher_title = "Homescreen";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut *window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };
}

fn authenticate(app_config: Res<AppConfig>, session: Res<Session>, client: BevyReqwest) {
    debug!("Entered authenticate");
    get_nonce(
        client,
        app_config.game_server_url.clone(),
        session.token.clone(),
    );
}

fn get_server_cookie(
    mut client: BevyReqwest,
    game_server_url: String,
    join_token: String,
    server_nonce: String,
) {
    let url = get_server_path(
        &game_server_url,
        game_server_structs::ServerAction::FinalLoginStep,
    );
    let req = client
        .post(url)
        .json(&FinalLoginStepRequest {
            join_token,
            server_nonce,
        })
        .build()
        .unwrap();
    client
        .send(req)
        .on_json_response(
            move |resp: On<JsonResponse<FinalLoginStepResponse>>,
                  mut next_state: ResMut<NextState<GameServerState>>,
                  mut server_cookie_resource: ResMut<ServerCookie>| {
                let resp = resp.data.clone();
                match resp {
                    FinalLoginStepResponse::Success(final_login_step_success) => {
                        debug!("Step 3 FinalLoginStepResponse: success: {final_login_step_success:#?}");
                        *server_cookie_resource =
                            ServerCookie(Some(final_login_step_success.cookie_token.clone()));
                        next_state.set(GameServerState::HomeScreen);
                    }
                    FinalLoginStepResponse::Failure(final_login_step_error) => {
                        debug!("final_login_step_error: {final_login_step_error:#?}");
                    }
                }
            },
        ) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
fn get_join_token(
    mut client: BevyReqwest,
    auth_server_url: String,
    nonce: String,
    session_token: String,
) {
    debug!("Entering get_join_token()");
    let url = auth_system_structs::public_api::get_full_url(
        auth_server_url.to_string(),
        auth_system_structs::public_api::PublicAPIAction::CreateJoinToken,
    );
    let req = client
        .post(url)
        .json(&CreateJoinTokenRequest {
            session_token: session_token.clone(),
            server_nonce: nonce.clone(),
        })
        .build()
        .unwrap();
    client
        .send(req)
        .on_json_response(
            move |resp: On<JsonResponse<CreateJoinTokenResponse>>,
                  client: BevyReqwest,
                  app_config: Res<AppConfig>| {
                let join_token = resp.data.clone();
                match join_token {
                    CreateJoinTokenResponse::Success(create_join_token_success) => {
                        debug!("Step 2 success: {create_join_token_success:#?}");
                        get_server_cookie(
                            client,
                            app_config.game_server_url.clone(),
                            create_join_token_success.join_token.clone(),
                            nonce.clone(),
                        );
                    }
                    CreateJoinTokenResponse::Error(create_join_token_error) => {
                        debug!("Step 2 get join token error: {create_join_token_error:#?}");
                    }
                }
            },
        ) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}

fn get_nonce(mut client: BevyReqwest, game_server_url: String, session_token: String) {
    debug!("Entered get_nonce");
    // step 1: get nonce from server
    let url = get_server_path(
        &game_server_url,
        game_server_structs::ServerAction::InitLogin,
    );
    let req = client.post(url).build().unwrap();
    client
        .send(req)
        .on_json_response(
            move |req: On<JsonResponse<ServerNonceResponse>>,
                  client: BevyReqwest,
                  app_config: Res<AppConfig>| {
                let nonce = req.data.clone();
                debug!("Step 1 success: nonce: {nonce:?}");
                get_join_token(
                    client,
                    app_config.auth_server_url.clone(),
                    nonce.0,
                    session_token.clone(),
                );
            },
        ) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
