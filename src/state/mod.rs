use bevy::prelude::*;

mod code_page;
mod game_data;

pub use code_page::CurrentCodePage;
pub use game_data::{GameData, GameLoadState};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameLoadState>()
            .add_observer(game_data::on_loaded)
            .add_observer(code_page::on_changed);
    }
}
