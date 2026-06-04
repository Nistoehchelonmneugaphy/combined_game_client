use auth_system_structs::public_api::{LoginErrorKind, LoginSuccess};
use bevy::prelude::*;
use shared::AppState;
use std::sync::{Arc, Mutex};


#[derive(Resource, Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub focused: FormField,
}
#[derive(Default, PartialEq)]
pub enum FormField {
    #[default]
    Username,
    Password,
}

#[derive(Component)]
pub struct UsernameInputFieldCursor;
#[derive(Component)]
pub struct PasswordInputFieldCursor;

#[derive(Component)]
pub struct UsernameInput;
#[derive(Component)]
pub struct PasswordInput;

#[derive(Component)]
pub struct UsernameText;

#[derive(Component)]
pub struct PasswordText;

#[derive(Component)]
pub struct LoginButton;

#[derive(Component)]
pub struct ErrorLabel;
#[derive(Component)]
pub struct ServerAddressInput;
#[derive(Component)]
pub struct ConnectButton;


#[derive(Resource)]
pub struct CursorBlink(pub Timer);
impl Default for CursorBlink {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource, Clone)]
pub struct LoginResultQueue(pub Arc<Mutex<Vec<Result<LoginSuccess, LoginErrorKind>>>>);

impl Default for LoginResultQueue {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
}

#[derive(Resource)]
pub struct Session {
    pub token: String,
    pub user_id: String,
}


#[derive(SubStates, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[source(AppState = AppState::Launcher)]
pub enum LauncherState {
    #[default]
    Login,
    ServerSelect,
}
