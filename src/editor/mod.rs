use atlaste_lcf::MapUnitAsset;
use bevy::prelude::*;

pub mod camera;
pub mod map_view;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_pancam::PanCamPlugin)
            .add_systems(Startup, (camera::setup, map_view::setup))
            .add_systems(
                Update,
                (
                    camera::check_movement.before(bevy_pancam::PanCamSystems),
                    map_view::on_map_unit_load.run_if(on_message::<AssetEvent<MapUnitAsset>>),
                    map_view::on_image_load.run_if(on_message::<AssetEvent<Image>>),
                ),
            )
            .add_observer(map_view::on_add)
            .add_observer(map_view::setup_view);
    }
}
