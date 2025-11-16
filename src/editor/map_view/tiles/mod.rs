use bevy::prelude::*;

mod fallback;
mod loading;
mod setup;

pub use fallback::Fallback;
pub use loading::MapChipSet;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fallback::setup)
            .add_systems(Update, loading::check)
            .add_observer(loading::start_on_add_map)
            .add_observer(setup::on_add_chipset);
    }
}
