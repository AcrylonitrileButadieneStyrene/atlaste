use bevy::prelude::*;

mod code_page;
mod game_state;
mod tool_state;
pub use code_page::{CodePage, CurrentCodePage};
pub use game_state::{GamePath, GameState};
pub use tool_state::ToolState;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<ToolState>()
            .add_systems(
                Update,
                game_state::load.run_if(resource_exists_and_changed::<GamePath>),
            );
    }
}
