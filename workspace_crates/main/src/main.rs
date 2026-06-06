use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::{AppConfig, AppState};
use startup::StartupPlugin;

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    println!("Hello, world!");
    dotenvy::dotenv().ok();
    let app_config = AppConfig {
        auth_server_url: env!("AUTH_SERVER_URL").to_string(),
    };
    let mut app = App::new();
    app.insert_resource(app_config);
    // create default window will be changed later
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Placeholder title".into(),
            resolution: WindowResolution::new(800, 600),
            ..default()
        }),
        ..default()
    }))
    .init_state::<AppState>()
    .add_plugins((StartupPlugin, LauncherPlugin,));
    app.run();
}
