use std::{ops::Deref, sync::Arc};

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum GameLoadState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Resource)]
pub struct GameData(pub Arc<atlaste_lcf::Game>);

impl Deref for GameData {
    type Target = Arc<atlaste_lcf::Game>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn on_loaded(
    loaded: On<atlaste_lcf::Loaded>,
    mut commands: Commands,
    mut next: ResMut<NextState<GameLoadState>>,
) {
    commands.insert_resource(GameData(loaded.0.clone()));
    next.set(GameLoadState::Loaded);
}
