use bevy::prelude::*;

use crate::state::GameData;

pub mod chipset;
pub mod map_unit;
pub mod setup;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, chipset::setup)
            .add_systems(
                Update,
                (map_unit::check_load, chipset::check_load).run_if(resource_exists::<GameData>),
            )
            .add_observer(on_add)
            .add_observer(setup::on_setup_tiles)
            .add_observer(setup::on_setup_background);
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
        map_unit::MapUnit(map),
        map_unit::Loading,
        Transform::default(),
        Visibility::default(),
    ));
}
