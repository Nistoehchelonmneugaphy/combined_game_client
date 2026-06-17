use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use auth_system_structs::public_api::{
    LoginErrorKind, LoginRequest, LoginResponse,
    PublicAPIAction::{self},
};
use bevy::prelude::*;
use shared::{AppConfig, CursorBlink, Session, set_title, spawn_text_field};

use crate::structs::LauncherState;

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
pub struct PasswordInput;

#[derive(Component)]
pub struct PasswordText;

#[derive(Component)]
pub struct UsernameInput;


#[derive(Component)]
pub struct UsernameText;


#[derive(Component)]
pub struct LoginButton;

#[derive(Component)]
pub struct ErrorLabel;

#[derive(Component)]
pub struct ConnectButton;


pub struct CouldNotContactAuthServer;

#[derive(Resource, Clone)]
pub struct LoginResultQueue(pub Arc<Mutex<Vec<Result<LoginResponse, CouldNotContactAuthServer>>>>);

impl Default for LoginResultQueue {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
}

pub fn add_launcher_login_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_text_input,
            blink_cursor,
            handle_login_button,
            poll_login_response,
            handle_server_connect,
        )
            .chain()
            .run_if(in_state(LauncherState::Login)),
    );
}
pub fn setup_launcher_login_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let launcher_title = "Launcher";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut *window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };

    commands.spawn((
        DespawnOnExit(LauncherState::Login),
        AudioPlayer::new(asset_server.load("audio/launcher/ambience/launcher_2.mp3")),
        PlaybackSettings::LOOP,
    ));
    commands.init_resource::<LoginForm>();
    commands.init_resource::<CursorBlink>();
    commands.init_resource::<LoginResultQueue>();

    // Root
    commands
        .spawn((
            DespawnOnExit(LauncherState::Login),
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
                LauncherState::Login,
            );
            spawn_text_field(
                p,
                "Password",
                PasswordInput,
                PasswordText,
                PasswordInputFieldCursor,
                LauncherState::Login,
            );

            // Login button
            p.spawn((
                DespawnOnExit(LauncherState::Login),
                LoginButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Login!"));

            // Error label (hidden until needed)
            p.spawn((
                DespawnOnExit(LauncherState::Login),
                ErrorLabel,
                Text::new(""),
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
        });
}

pub fn teardown_launcher_login_ui(mut commands: Commands) {
    // I assume that the DespawnOnExit takes care of all spawn'ed entities.
    // So, I guess I only need to do something like this... Remove the LoginForm? Since DespawnOnExit only works on commands.spawn. I think?..
    commands.remove_resource::<LoginForm>();
    commands.remove_resource::<CursorBlink>();
    commands.remove_resource::<LoginResultQueue>();
}

pub fn handle_text_input(
    mut form: ResMut<LoginForm>,
    mut evr: MessageReader<bevy::input::keyboard::KeyboardInput>,
    username_field_interaction_query: Query<&Interaction, With<UsernameInput>>,
    password_field_interaction_query: Query<&Interaction, With<PasswordInput>>,
    #[cfg(target_os = "android")] anywhere_else_interaction_query: Query<
        &Interaction,
        (Without<UsernameInput>, Without<PasswordInput>),
    >,
    mut username_text: Query<&mut Text, (With<UsernameText>, Without<PasswordText>)>,
    mut password_text: Query<&mut Text, (With<PasswordText>, Without<UsernameText>)>,
) {
    let username_pressed = username_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    let password_pressed = password_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    #[cfg(target_os = "android")]
    let anywhere_else_pressed = anywhere_else_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);

    if username_pressed {
        form.focused = FormField::Username;
    } else if password_pressed {
        form.focused = FormField::Password;
    }
    #[cfg(target_arch = "wasm32")]
    {
        if web::is_mobile()
            && !web_mobile_input::is_keyboard_shown()
            && (username_pressed || password_pressed)
        {
            web_mobile_input::show_keyboard();
        }
    }
    #[cfg(target_os = "android")]
    {
        if username_pressed {
            android_mobile_input::show_keyboard(&form.username, "Username");
        } else if password_pressed {
            android_mobile_input::show_keyboard(&form.password, "Password");
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
                FormField::Username => &mut form.username,
                FormField::Password => &mut form.password,
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
                FormField::Username => &mut form.username,
                FormField::Password => &mut form.password,
            };
            target.push_str(s);
        }
    }

    // Sync to UI - now these queries will actually find the entities
    if let Ok(mut text) = username_text.single_mut() {
        text.0 = form.username.clone();
    }
    if let Ok(mut text) = password_text.single_mut() {
        text.0 = "*".repeat(form.password.len());
    }
}

pub fn blink_cursor(
    time: Res<Time>,
    mut blink: ResMut<CursorBlink>,
    form: Res<LoginForm>,
    mut username_cursor: Query<
        &mut Visibility,
        (
            With<UsernameInputFieldCursor>,
            Without<PasswordInputFieldCursor>,
        ),
    >,
    mut password_cursor: Query<
        &mut Visibility,
        (
            With<PasswordInputFieldCursor>,
            Without<UsernameInputFieldCursor>,
        ),
    >,
) {
    blink.0.tick(time.delta());

    let (mut active, mut inactive) = match form.focused {
        FormField::Username => (
            username_cursor.single_mut().unwrap(),
            password_cursor.single_mut().unwrap(),
        ),
        FormField::Password => (
            password_cursor.single_mut().unwrap(),
            username_cursor.single_mut().unwrap(),
        ),
    };

    // Inactive field cursor always hidden
    *inactive = Visibility::Hidden;

    // Active field cursor blinks
    if blink.0.just_finished() {
        *active = match *active {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub fn handle_login_button(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut queue: ResMut<LoginResultQueue>,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<LoginButton>)>,
    form: Res<LoginForm>,
    config: Res<AppConfig>,
    mut error_q: Query<&mut Text, With<ErrorLabel>>,
) {
    for interaction in &interaction_q {
        let config = config.clone();
        if *interaction != Interaction::Pressed {
            continue;
        }

        #[cfg(target_arch = "wasm32")]
        {
            if web::is_mobile() && web_mobile_input::is_keyboard_shown() {
                web_mobile_input::hide_keyboard();
            }
        }

        if let Ok(mut text) = error_q.single_mut() {
            **text = "".to_string();
        }
        let (username, password) = (form.username.clone(), form.password.clone());

        login_request(&mut queue, &config.auth_server_url, &username, &password);
        commands.spawn((
            DespawnOnExit(LauncherState::Login),
            AudioPlayer::new(asset_server.load("audio/launcher/button_press/login.mp3")),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn login_request(
    queue: &mut ResMut<LoginResultQueue>,
    auth_server_url: &str,
    username: &str,
    password: &str,
) {
    let body = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    let Ok(body_vec) = serde_json::to_vec(&body) else {
        return;
    };
    let queue_clone = queue.clone();

    let request = ehttp::Request::post(
        auth_system_structs::public_api::get_full_url(
            auth_server_url.into(),
            PublicAPIAction::Login,
        ),
        body_vec,
    )
    .with_timeout(Some(Duration::from_secs(7)));

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let Ok(mut queue) = queue_clone.0.lock() else {
            error!("Could not get queue lock");
            return;
        };
        match result {
            Ok(r) => {
                let Ok(resp) = serde_json::from_slice::<LoginResponse>(&r.bytes) else {
                    error!("Could not shape response into LoginResponse.");
                    error!(
                        "Response bytes: {:#?}, response headers: {:#?}",
                        String::from_utf8_lossy(&r.bytes),
                        &r.headers
                    );
                    queue.push(Err(CouldNotContactAuthServer));
                    return;
                };
                info!("login resp: {resp:#?}");
                queue.push(Ok(resp));
            }
            Err(_) => {
                warn!("req err");
                queue.push(Err(CouldNotContactAuthServer));
            }
        }
    });
    use bevy::prelude::*;
}

pub fn poll_login_response(
    queue: ResMut<LoginResultQueue>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<LauncherState>>,
    mut error_q: Query<&mut Text, With<ErrorLabel>>,
) {
    let mut q = queue.0.lock().unwrap();

    for result in q.drain(..) {
        match result {
            Err(_e) => {
                if let Ok(mut text) = error_q.single_mut() {
                    **text = "Couldn't contact the auth server".to_string();
                }
            }
            Ok(login_response) => match login_response {
                LoginResponse::Success(success) => {
                    commands.insert_resource(Session {
                        token: success.token,
                        user_id: success.user_id,
                    });
                    if let Ok(mut text) = error_q.single_mut() {
                        **text = "Login successful".to_string();
                    }
                    debug!("Switching to ServerSelect state");
                    next_state.set(LauncherState::ServerSelect);
                }
                LoginResponse::Error(err) => {
                    let err_kind = err.kind;
                    warn!("Login failed: {err_kind:?}");
                    let msg = match err_kind {
                        LoginErrorKind::InvalidCredentials => "Invalid username or password.",
                        LoginErrorKind::SubscriptionExpired => "Subscription expired.",
                    };
                    if let Ok(mut text) = error_q.single_mut() {
                        **text = msg.to_string();
                    }
                }
            },
        }
    }
    use bevy::prelude::*;
}

pub fn handle_server_connect(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    session: Option<Res<Session>>,
) {
    for interaction in &interaction_q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(session) = &session else {
            continue;
        };
        info!("{} {}", session.token, session.user_id);
        // When I will combine the launcher into a single app together with the server client and match client, I will add here a state switch to server and set the session resource or something.
    }
}
