use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::{AppConfig, AppState};

fn main() {
    let app_config = AppConfig {
        auth_server_url: std::env::var("AUTH_SERVER_URL")
            .expect("AUTH_SERVER_URL must be set in .env"),
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
