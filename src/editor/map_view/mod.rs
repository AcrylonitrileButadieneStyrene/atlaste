use bevy::prelude::*;

use crate::state::GameData;

pub mod background;
pub mod events;
pub mod map_unit;
pub mod panorama;
pub mod tiles;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            map_unit::Plugin,
            panorama::Plugin,
            tiles::Plugin,
            events::Plugin,
        ))
        .add_observer(on_add)
        .add_observer(background::on_add_map_unit);
    }
}

fn on_add(
    trigger: On<atlaste_ui::sections::map_tree::EntryClicked>,
    mut commands: Commands,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    let map = asset_server.load(
        game.game_dir
            .resolve(&format!("Map{:0>4}.lmu", trigger.0))
            .unwrap(),
    );

    commands.spawn((
        map_unit::Loading(map),
        Transform::default(),
        Visibility::default(),
    ));
}
