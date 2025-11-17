use std::sync::Arc;

use atlaste_image::IndexedImage;
use bevy::{asset::LoadState, prelude::*};

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
};

#[derive(Component)]
pub struct Loading {
    png: Handle<IndexedImage>,
    bmp: Handle<IndexedImage>,
}

#[derive(Component)]
pub struct MapChipSet(pub Handle<Image>);

pub fn start_on_add_map(
    spawn: On<Add, MapUnit>,
    map: Query<&MapUnit>,
    game: Res<GameData>,
    code_page: Res<CurrentCodePage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) -> Result {
    let MapUnit { map, .. } = map.get(spawn.entity)?;

    let id = map.chipset.unwrap_or(1);
    let bytes = &game.database.chipsets[id as usize - 1].file;
    let file = code_page.0.to_encoding().decode(bytes).0.to_string();
    let base = game.game_dir.join("ChipSet/").join(file);

    // bevy 18 will add a setting to loading images but it does not help me because chipsets are not 1x480, they are 30x16
    let texture = Loading {
        png: asset_server.load(base.with_added_extension("png")),
        bmp: asset_server.load(base.with_added_extension("bmp")),
    };

    commands.entity(spawn.entity).insert(texture);

    Ok(())
}

enum State {
    Loading,
    Loaded(Handle<Image>, Option<Arc<[u8]>>),
}

pub fn check(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Loading)>,
    fallback: Res<super::Fallback>,
    indexed_images: Res<Assets<IndexedImage>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    for (entity, Loading { png, bmp }) in query.iter() {
        let handle = match asset_server
            .get_load_state(png)
            .zip(asset_server.get_load_state(bmp))
        {
            Some((LoadState::Loaded, _)) => Some(png.clone()),
            Some((_, LoadState::Loaded)) => Some(bmp.clone()),
            // either is loading but neither is loaded
            Some((LoadState::Loading, _) | (_, LoadState::Loading)) => continue,
            None | Some((LoadState::NotLoaded, _) | (_, LoadState::NotLoaded)) => {
                error!("ChipSet did not start loading");
                None
            }
            Some((LoadState::Failed(_), LoadState::Failed(_))) => None,
        }
        .and_then(|handle| {
            // it is so hopeless
            let IndexedImage { image, palette } = indexed_images.get(&handle).unwrap();
            match asset_server.get_load_state(image) {
                Some(LoadState::Loaded) => Some(State::Loaded(image.clone(), palette.clone())),
                Some(LoadState::Loading) => Some(State::Loading),
                None | Some(LoadState::NotLoaded | LoadState::Failed(_)) => None,
            }
        });

        let handle = match handle {
            Some(State::Loaded(handle, palette)) => {
                let alpha_key = palette.map(|palette| {
                    (palette[0] as u32) << 16 | (palette[1] as u32) << 8 | (palette[2] as u32)
                });

                let mut image = images.get_mut(&handle).unwrap();
                atlaste_image::image_to_chipset(&mut image, alpha_key);

                handle
            }
            Some(State::Loading) => continue,
            None => fallback.0.clone(),
        };

        commands
            .entity(entity)
            .insert(MapChipSet(handle))
            .remove::<Loading>();
    }
}
