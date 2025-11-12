use bevy::prelude::*;

mod code_page;
mod game_data;

pub use code_page::CurrentCodePage;
pub use game_data::GameData;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_observer(game_data::on_loaded)
            .add_observer(code_page::on_changed);
    }
}
