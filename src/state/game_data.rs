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
    codepage: Res<super::CurrentCodePage>,
) {
    commands.insert_resource(GameData(loaded.0.clone()));
    next.set(GameLoadState::Loaded);
    commands.trigger(atlaste_ui::sections::map_tree::AddEntries(
        codepage.0,
        loaded
            .0
            .map_tree
            .maps
            .iter()
            .map(|(id, map)| atlaste_ui::sections::map_tree::Entry {
                id: *id,
                parent: map.parent,
                name: map.name.clone(),
                indentation: map.indentation,
            })
            .collect(),
    ));
}
