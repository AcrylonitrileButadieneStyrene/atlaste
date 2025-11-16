use bevy::{asset::LoadState, prelude::*, render::render_resource::Extent3d};

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
};

#[derive(Component)]
pub struct Loading {
    png: Handle<Image>,
    bmp: Handle<Image>,
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

pub fn check(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Loading)>,
    fallback: Res<super::Fallback>,
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
        .inspect(|handle| {
            let image = images.get_mut(handle).unwrap();
            let pixels = image.data.take().unwrap();

            // wgpu reads the elements as lines on the image instead of as squares, so they need to be repacked
            image.data = Some(fix_pixels(pixels));
            image.reinterpret_size(Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 480,
            });
        })
        .unwrap_or_else(|| fallback.0.clone());

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
