use bevy::prelude::*;

pub mod map_view;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_observer(map_view::on_add)
            .add_observer(map_view::on_loaded)
            .add_systems(Update, map_view::process_loading);
    }
}
