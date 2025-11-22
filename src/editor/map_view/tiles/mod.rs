use atlaste_asset::{DualR2kImage, DualR2kImageLoaded, R2kImage};
use bevy::{prelude::*, ui_widgets::observe};

mod fallback;
mod setup;

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
};

#[derive(Component)]
pub struct MapChipSet(pub Handle<Image>);

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fallback::setup)
            .add_observer(setup::on_add_chipset)
            .add_observer(start_on_add_map);
    }
}

pub fn start_on_add_map(
    spawn: On<Add, MapUnit>,
    map: Query<&MapUnit>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
    mut commands: Commands,
) -> Result {
    let MapUnit { map, .. } = map.get(spawn.entity)?;

    let bytes = &game.database.chipsets[map.chipset as usize - 1].file;
    let file = code_page.0.to_encoding().decode(bytes).0.to_string();
    commands.entity(spawn.entity).insert((
        DualR2kImage {
            base: game.game_dir.resolve("ChipSet").unwrap(),
            file,
        },
        observe(
            |loaded: On<DualR2kImageLoaded>,
             r2k_images: Res<Assets<R2kImage>>,
             mut images: ResMut<Assets<Image>>,
             fallback: Res<fallback::Fallback>,
             mut commands: Commands| {
                let handle = match &loaded.status {
                    Ok(()) => {
                        let r2k = r2k_images.get(&loaded.handle).unwrap();
                        atlaste_asset::chipset::image_to_chipset(
                            images.get_mut(&r2k.image).unwrap(),
                            r2k.alpha_key,
                        );
                        r2k.image.clone()
                    }
                    Err(None) => {
                        error!("Failed to load ChipSet: did not start loading.");
                        fallback.0.clone()
                    }
                    Err(Some(reason)) => {
                        error!("Failed to load ChipSet: {reason}");
                        fallback.0.clone()
                    }
                };

                commands.entity(loaded.entity).insert(MapChipSet(handle));
            },
        ),
    ));

    Ok(())
}
