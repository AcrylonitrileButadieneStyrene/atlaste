use bevy::prelude::*;

mod side;
mod top;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiState>()
            .add_systems(Update, (top::draw, side::draw).chain_ignore_deferred());
    }
}

#[derive(Default, Resource)]
pub struct EguiState {
    pub game_dir: std::path::PathBuf,
    pub code_page: Option<crate::state::Encoding>,
}
