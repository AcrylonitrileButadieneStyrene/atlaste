use bevy::prelude::*;

pub mod camera;
pub mod map_view;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_pancam::PanCamPlugin)
            .add_event::<map_view::Loaded>()
            .add_systems(Startup, camera::setup)
            .add_systems(
                Update,
                (
                    map_view::process_loading,
                    map_view::process_loaded.run_if(on_event::<map_view::Loaded>),
                ),
            )
            .add_observer(map_view::on_add);
    }
}
