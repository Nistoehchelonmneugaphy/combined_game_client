use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::{AppConfig, AppState};

#[bevy_main]
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    println!("Hello, world!");
    dotenvy::dotenv().ok();
    let app_config = AppConfig {
        auth_server_url: "http://127.0.0.1:55555".to_string(),
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
    .add_plugins((LauncherPlugin,));
    app.run();
}
