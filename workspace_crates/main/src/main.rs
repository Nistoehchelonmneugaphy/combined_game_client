use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::AppState;

fn main() {
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
    .add_plugins((LauncherPlugin,));
    app.run();
}
