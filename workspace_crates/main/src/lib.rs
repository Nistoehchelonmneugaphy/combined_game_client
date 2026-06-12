use bevy::log::LogPlugin;
use bevy::{prelude::*, window::WindowResolution};
use game_server_client::GameServerClientPlugin;
use launcher::LauncherPlugin;
use shared::AppState;
use startup::StartupPlugin;

#[cfg(target_arch = "wasm32")]
use web_mobile_input;

#[bevy_main]
pub fn main() {
    println!("hey there!!! :)");

    // Without this, error console.logs in wasm builds are rather unhelpful, but adding this adds helpful stack traces of where errors occurred
    #[cfg(target_arch = "wasm32")]
    web::make_console_errors_more_helpful();

    let mut app = App::new();
    app.add_systems(Startup, add_camera_2d);
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(web_mobile_input::MobileInputPlugin);

    #[cfg(target_os = "android")]
    {
        app.insert_resource(bevy::winit::WinitSettings::mobile());
        app.add_systems(
            Update,
            handle_lifetime.run_if(any_with_component::<AudioSink>),
        );
        app.add_plugins(android_mobile_input::AndroidMobileInputPlugin);
    }

    // create default window will be changed later
    let window_plugin_config = WindowPlugin {
        primary_window: Some(Window {
            title: "Placeholder title".into(),
            resolution: WindowResolution::new(800, 600),

            #[cfg(target_arch = "wasm32")]
            canvas: Some(format!("#{}", web::BEVY_CANVAS_ID)),
            #[cfg(target_os = "android")]
            resizable: false,
            #[cfg(target_os = "android")]
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),

            ..default()
        }),
        ..default()
    };
    let log_plugin_config = LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "wgpu=error,bevy=info,main=warn,launcher=debug,shared=debug,startup=debug,web_mobile_input=debug,web=debug,game_server_client=debug"
            .to_string(),
        ..default()
    };
    // This fixed meta errors on wasm and allows audio and other assets to load. Yay
    let asset_server_plugin_config = AssetPlugin {
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    };
    app.add_plugins(
        DefaultPlugins
            .set(window_plugin_config)
            .set(log_plugin_config)
            .set(asset_server_plugin_config),
    )
    .init_state::<AppState>()
    .add_plugins((StartupPlugin, LauncherPlugin, GameServerClientPlugin));
    app.run();
}

fn add_camera_2d(mut commands: Commands) {
        commands.spawn(Camera2d);
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.

#[cfg(target_os = "android")]
fn handle_lifetime(
    mut app_lifecycle_reader: MessageReader<AppLifecycle>,
    music_controller: Single<&AudioSink>,
) {
    use bevy::window::AppLifecycle;
    for app_lifecycle in app_lifecycle_reader.read() {
        match app_lifecycle {
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => {}
            AppLifecycle::Suspended => music_controller.pause(),
            AppLifecycle::Running => music_controller.play(),
        }
    }
}
