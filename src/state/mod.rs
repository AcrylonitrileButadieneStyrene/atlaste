use bevy::prelude::*;

mod code_page;
pub use code_page::{CodePage, CurrentCodePage};

mod game_data;
pub use game_data::{GameData, GameLoadState};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameLoadState>()
            .add_observer(game_data::on_loaded);
    }
}
