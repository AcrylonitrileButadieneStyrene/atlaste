use atlaste_lcf::MapUnitAsset;
use bevy::{
    asset::{LoadState, RenderAssetUsages},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Resource)]
pub struct Fallback(pub Handle<Image>);

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let texture = images.add(Image::new_fill(
        Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 480,
        },
        TextureDimension::D2,
        &[0x00, 0xFF, 0x00, 0x00],
        TextureFormat::bevy_default(),
        RenderAssetUsages::RENDER_WORLD,
    ));
    commands.insert_resource(Fallback(texture));
}

#[derive(Component)]
pub enum Loading {
    Regular {
        png: Handle<Image>,
        bmp: Handle<Image>,
    },
    Fallback,
}

pub fn check_load(
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &super::map_unit::Loading, &Loading)>,
    fallback: Res<Fallback>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    map_units: Res<Assets<MapUnitAsset>>,
) {
    for (entity, map_unit, chipset) in query.iter() {
        let handle = match chipset {
            Loading::Regular { png, bmp } => {
                match asset_server
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
            }
            Loading::Fallback => None,
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
            .remove::<super::map_unit::Loading>()
            .remove::<Loading>();

        commands.trigger(super::setup::Spawn {
            entity,
            map: map_units.get(&map_unit.0).unwrap().0.clone(),
            chipset: handle,
        });
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
