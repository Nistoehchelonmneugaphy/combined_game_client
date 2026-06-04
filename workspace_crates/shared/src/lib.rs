use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Launcher,
    Server,
    Match,
}