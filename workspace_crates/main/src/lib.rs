use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::{AppState};
use startup::StartupPlugin;
use bevy::log::{info};


pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    info!("Hello, world!");
    let mut app = App::new();
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
