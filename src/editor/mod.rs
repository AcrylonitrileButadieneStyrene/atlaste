use bevy::prelude::*;

pub mod camera;
pub mod map_view;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_pancam::PanCamPlugin)
            .add_systems(Startup, camera::setup)
            .add_systems(
                Update,
                (
                    camera::check_movement.before(bevy_pancam::PanCamSystemSet),
                    map_view::process_loading,
                ),
            )
            .add_observer(map_view::on_add)
            .add_observer(map_view::setup_view);
    }
}
