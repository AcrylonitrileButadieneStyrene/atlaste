use bevy::prelude::*;

pub mod camera;
pub mod map_view;
pub mod select;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((bevy_pancam::PanCamPlugin, map_view::Plugin))
            .add_systems(Startup, (camera::setup, select::setup))
            .add_systems(
                Update,
                (
                    camera::disable_when_hovering_over_ui.before(bevy_pancam::PanCamSystems),
                    select::handle_keypress,
                ),
            )
            .add_observer(select::on_add)
            .add_observer(select::on_click);
    }
}
