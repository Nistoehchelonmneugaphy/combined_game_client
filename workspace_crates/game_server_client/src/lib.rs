use std::sync::{Arc, Mutex};

use auth_system_structs::public_api::{CreateJoinTokenRequest, CreateJoinTokenResponse};
use bevy::prelude::*;
use bevy_mod_reqwest::{BevyReqwest, ReqwestErrorEvent, ReqwestPlugin, ReqwestResponseEvent};
use game_server_structs::{FinalLoginStepRequest, FinalLoginStepResponse, ServerNonceResponse, get_server_path};
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
        app.insert_resource(SharedAuthenticationStepState(Arc::new(Mutex::new(
            AuthenticationStepState::GetServerNonce,
        ))));

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

#[derive(Debug, Clone, PartialEq)]
enum AuthenticationStepState {
    GetServerNonce,
    GetJoinToken {
        server_nonce: String,
        session_token: String,
    },
    GetServerCookie {
        join_token: String,
        server_nonce: String,
    },
    
    Complete { server_cookie: String },
    Failed,
}
#[derive(Resource, Clone)]
struct SharedAuthenticationStepState(Arc<Mutex<AuthenticationStepState>>);

impl SharedAuthenticationStepState {
    fn update_state(&self, new_state: AuthenticationStepState) {
        let Ok(mut guard) = self.0.lock() else { return };
        *guard = new_state;
    }
}

fn authenticate(
    app_config: Res<AppConfig>,
    session: Res<Session>,
    client: BevyReqwest,
    auth_state: ResMut<SharedAuthenticationStepState>,
) {
    debug!("Entered authenticate");
    let app_config = app_config.clone();
    let session = session.clone();
    let game_server_url = app_config.game_server_url;
    let auth_server_url = app_config.auth_server_url;
    let Ok(guard) = auth_state.0.lock() else {
        return;
    };
    match &*guard {
        AuthenticationStepState::GetServerNonce => {
            let state = auth_state.clone();
            get_nonce(
                state.clone(),
                client,
                game_server_url.clone(),
                session.token.clone(),
            );
        }
        AuthenticationStepState::GetJoinToken {
            server_nonce: nonce,
            session_token,
        } => {
            let state = auth_state.clone();
            get_join_token(
                state.clone(),
                client,
                auth_server_url,
                nonce.clone(),
                session_token.clone(),
            );
        }
        AuthenticationStepState::GetServerCookie {
            join_token,
            server_nonce,
        } => {
            let state = auth_state.clone();
            get_server_cookie(
                state.clone(),
                client,
                game_server_url,
                join_token.clone(),
                server_nonce.clone(),
            );
        }
        AuthenticationStepState::Complete { server_cookie} => {
            debug!("Authentication success, server_cookie: {server_cookie}");
        },
        AuthenticationStepState::Failed => {
            debug!("Authentication failure");
        },
    }
}

fn get_server_cookie(
    state: SharedAuthenticationStepState,
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
        .on_response(move |req: On<ReqwestResponseEvent>| {
            let req = req.event();
            let Ok(resp) = serde_json::from_slice::<FinalLoginStepResponse>(&req.body()) else {
                error!("Could not shape response into FinalLoginStepResponse.");
                error!(
                    "Response bytes: {:#?}, response headers: {:#?}",
                    String::from_utf8_lossy(&req.body()),
                    &req.response_headers()
                );
                return;
            };
            match resp {
                FinalLoginStepResponse::Success(final_login_step_success) => {
                    debug!("FinalLoginStepResponse: success: {final_login_step_success:#?}");
                    state.update_state(AuthenticationStepState::Complete { server_cookie: final_login_step_success.cookie_token });
                },
                FinalLoginStepResponse::Failure(final_login_step_error) => {
                    debug!("final_login_step_error: {final_login_step_error:#?}");
                    state.update_state(AuthenticationStepState::Failed);
                },
            }
        }) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
fn get_join_token(
    state: SharedAuthenticationStepState,
    mut client: BevyReqwest,
    auth_server_url: String,
    nonce: String,
    session_token: String,
) {
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
        .on_response(move |req: On<ReqwestResponseEvent>| {
            let req = req.event();
            let Ok(resp) = serde_json::from_slice::<CreateJoinTokenResponse>(&req.body()) else {
                error!("Could not shape response into CreateJoinTokenResponse.");
                error!(
                    "Response bytes: {:#?}, response headers: {:#?}",
                    String::from_utf8_lossy(&req.body()),
                    &req.response_headers()
                );
                return;
            };
            match resp {
                CreateJoinTokenResponse::Success(create_join_token_success) => {
                    debug!("Step 2 success: {create_join_token_success:#?}");
                    state.update_state(AuthenticationStepState::GetServerCookie {
                        join_token: create_join_token_success.join_token,
                        server_nonce: nonce.clone(),
                    });
                }
                CreateJoinTokenResponse::Error(create_join_token_error) => {
                    debug!("Step 2 get join token error: {create_join_token_error:#?}");
                }
            }
        }) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
fn get_nonce(
    state: SharedAuthenticationStepState,
    mut client: BevyReqwest,
    game_server_url: String,
    session_token: String,
) {
    // step 1: get nonce from server
    let url = get_server_path(
        &game_server_url,
        game_server_structs::ServerAction::InitLogin,
    );
    let req = client.post(url).build().unwrap();
    client
        .send(req)
        .on_response(move |req: On<ReqwestResponseEvent>| {
            let req = req.event();
            let Ok(nonce) = serde_json::from_slice::<ServerNonceResponse>(&req.body()) else {
                error!("Could not shape response into ServerNonceResponse.");
                error!(
                    "Response bytes: {:#?}, response headers: {:#?}",
                    String::from_utf8_lossy(&req.body()),
                    &req.response_headers()
                );
                return;
            };
            debug!("Step 1 success: nonce: {nonce:?}");
            state.update_state(AuthenticationStepState::GetJoinToken {
                server_nonce: nonce.0.clone(),
                session_token: session_token.clone(),
            });
        }) // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: On<ReqwestErrorEvent>| {
            let e = &trigger.event().error;
            debug!("bevy_mod_reqwest: req error: {e:?}");
        });
}
