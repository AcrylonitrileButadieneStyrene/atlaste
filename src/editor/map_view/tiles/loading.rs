use std::sync::Arc;

use atlaste_image::IndexedImage;
use bevy::{asset::LoadState, prelude::*, render::render_resource::Extent3d};

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
                let image = images.get_mut(&handle).unwrap();
                let mut pixels = image.data.take().unwrap();

                if let Some(palette) = palette {
                    let transparent =
                        (palette[0] as u32) << 16 | (palette[1] as u32) << 8 | (palette[2] as u32);
                    for pixel in pixels.chunks_exact_mut(4) {
                        let color = u32::from_be_bytes(pixel.try_into().unwrap()) >> 8;
                        if color == transparent {
                            pixel[3] = 0x00;
                        }
                    }
                }

                // wgpu reads the elements as lines on the image instead of as squares, so they need to be repacked
                image.data = Some(fix_pixels(pixels));
                image.reinterpret_size(Extent3d {
                    width: 16,
                    height: 16,
                    depth_or_array_layers: 480,
                });

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

// there is definitely a better way to do this but this worked first try so i will not be changing it
#[must_use]
fn fix_pixels(pixels: Vec<u8>) -> Vec<u8> {
    let mut new_pixels = Vec::with_capacity(pixels.len());

    for tile_index in 0..480 {
        let start_x = tile_index % 30;
        let start_y = tile_index / 30;
        for square_index in 0..256 {
            let add_x = square_index % 16;
            let add_y = square_index / 16;
            for byte in 0..4 {
                let pixel =
                    pixels[(start_x * 16 + add_x + (start_y * 16 + add_y) * 480) * 4 + byte];
                new_pixels.push(pixel);
            }
        }
    }

    new_pixels
}
