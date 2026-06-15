use bevy::asset::AssetLoader;
use bevy::prelude::*;
use shared::{AppConfig, AppConfigAsset, AppConfigHandle, AppState};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AppConfigAsset>()
            .init_asset_loader::<AppConfigJsonLoader>()
            .add_systems(OnEnter(AppState::Startup), start_loading)
            .add_systems(Update, check_loaded.run_if(in_state(AppState::Startup)));
    }
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("config.json");
    commands.insert_resource(AppConfigHandle(handle));
}

fn check_loaded(
    mut commands: Commands,
    handle: Res<AppConfigHandle>,
    assets: Res<Assets<AppConfigAsset>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(config) = assets.get(&handle.0) {
        debug!("Retrieved assets/config.json: {:#?}", config);
        let AppConfigAsset {
            auth_server_url,
            game_server_url,
            ws_game_server_url,
        } = config;
        let app_config = AppConfig {
            auth_server_url: auth_server_url.to_string(),
            game_server_url: game_server_url.to_string(),
            ws_game_server_url: ws_game_server_url.to_string(),
        };
        commands.insert_resource(app_config);
        debug!("Switching from AppState::Startup to AppState::Launcher");
        next_state.set(AppState::Launcher);
    }
}

// Asset loader
use bevy::asset::io::Reader;

#[derive(Default, TypePath)]
struct AppConfigJsonLoader;

impl AssetLoader for AppConfigJsonLoader {
    type Asset = AppConfigAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
