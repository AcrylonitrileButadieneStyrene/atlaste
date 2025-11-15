use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::state::GameData;

pub mod loading;
pub mod panorama;
pub mod setup;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<panorama::Material>::default())
            .add_systems(Startup, loading::chipset::setup)
            .add_systems(
                Update,
                (loading::map_unit::check_load, loading::chipset::check_load)
                    .run_if(resource_exists::<GameData>),
            )
            .add_observer(on_add)
            .add_observer(setup::tiles::on_spawn)
            .add_observer(setup::on_spawn_background)
            .add_observer(panorama::on_spawn);
    }
}

pub fn on_add(
    trigger: On<atlaste_ui::sections::map_tree::EntryClicked>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let map = asset_server.load(game.game_dir.join(format!("Map{:0>4}.lmu", trigger.0)));

    commands.spawn((
        loading::map_unit::Loading(map),
        Transform::default(),
        Visibility::default(),
    ));
}
