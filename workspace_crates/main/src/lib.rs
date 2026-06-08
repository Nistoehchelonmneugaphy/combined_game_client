use bevy::log::LogPlugin;
use bevy::{prelude::*, window::WindowResolution};
use launcher::LauncherPlugin;
use shared::AppState;
use startup::StartupPlugin;

#[cfg(target_arch = "wasm32")]
use web_mobile_input;

#[bevy_main]
pub fn main() {
    println!("hey there");
    #[cfg(target_arch = "wasm32")]
    web::console_error_panic_hook::set_once();
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_plugins(web_mobile_input::MobileInputPlugin);

    // create default window will be changed later
    let window_plugin_config = WindowPlugin {
        primary_window: Some(Window {
            title: "Placeholder title".into(),
            resolution: WindowResolution::new(800, 600),    

            // #[cfg(target_arch = "wasm32")]
            // prevent_default_event_handling: true,
            #[cfg(target_arch = "wasm32")]
            canvas: Some(format!("#{}", web::BEVY_CANVAS_ID)),

            ..default()
        }),
        ..default()
    };
    let log_plugin_config = LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "wgpu=error,bevy=info,main=warn,launcher=warn,shared=warn,startup=warn,web_mobile_input=warn,web=warn"
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
    .add_plugins((StartupPlugin, LauncherPlugin));
    app.run();
}
