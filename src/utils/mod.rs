use bevy::prelude::*;

pub mod unit_mesh;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, unit_mesh::setup);
    }
}
