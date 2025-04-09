use bevy::prelude::*;

mod code_page;
mod game_state;

pub use code_page::{CodePage, CurrentCodePage};
pub use game_state::{GamePath, GameState};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            game_state::load.run_if(resource_exists_and_changed::<GamePath>),
        );
    }
}
