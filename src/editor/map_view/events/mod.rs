use bevy::{prelude::*, sprite_render::Material2dPlugin, ui_widgets::observe};

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
    utils::unit_mesh::UnitRectangle,
};

mod material;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<material::Material>::default())
            .add_observer(on_add_map_unit);
    }
}

fn on_add_map_unit(
    event: On<Add, MapUnit>,
    query: Query<&MapUnit>,
    mut commands: Commands,
    codepage: Res<CurrentCodePage>,
    asset_server: Res<AssetServer>,
    rectange: Res<UnitRectangle>,
    game: Res<GameData>,
) -> Result {
    let MapUnit { map, .. } = query.get(event.entity)?;

    let container = commands
        .spawn((
            Name::new("Events"),
            ChildOf(event.entity),
            Transform::from_translation(Vec3::Z),
            Visibility::Inherited,
        ))
        .id();

    for event in &map.events {
        let page = &event.pages[0];

        let file = codepage
            .0
            .to_encoding()
            .decode(&page.graphic.file)
            .0
            .to_string();
        let charset = game
            .game_dir
            .resolve(&format!("CharSet/{file}.png")) // todo: also this one can also be a .bmp
            .unwrap();
        let options = u32::from_ne_bytes(
            material::Options::from_event(&page.graphic, page.animation_type).into_bytes(),
        );

        commands.spawn((
            ChildOf(container),
            Transform::from_scale(Vec3::new(1.5, 2., 1.)).with_translation(Vec3::new(
                event.x as f32 - map.width as f32 / 2.0 + 0.5,
                (map.height - event.y) as f32 - map.height as f32 / 2.0,
                (event.y + event.x) as f32 / 1000.,
            )),
            Mesh2d(rectange.0.clone()),
            Children::spawn_one((
                atlaste_asset::ObservedAsset {
                    handle: asset_server
                        .load::<atlaste_asset::R2kImage>(charset)
                        .untyped(),
                    despawn: true,
                },
                observe(
                    move |loaded: On<atlaste_asset::ObservedAssetLoaded>,
                          r2k_images: Res<Assets<atlaste_asset::R2kImage>>,
                          mut images: ResMut<Assets<Image>>,
                          parent: Query<&ChildOf>,
                          mut commands: Commands,
                          mut materials: ResMut<Assets<material::Material>>|
                          -> Result {
                        match &loaded.status {
                            Ok(()) => {
                                let r2k = r2k_images.get(&loaded.handle.clone().typed()).unwrap();
                                let image = images.get_mut(&r2k.image.clone()).unwrap();

                                if let Some(data) = image.data.as_mut() {
                                    atlaste_asset::chipset::chromakey(data, r2k.alpha_key);
                                }

                                commands.entity(parent.get(loaded.entity)?.parent()).insert(
                                    MeshMaterial2d(materials.add(material::Material {
                                        texture: r2k.image.clone(),
                                        options,
                                    })),
                                );
                            }
                            Err(None) => {
                                error!("CharSet did not start loading");
                            }
                            Err(Some(err)) => {
                                error!("Failed to load CharSet: {err}");
                            }
                        }

                        Ok(())
                    },
                ),
            )),
        ));
    }

    Ok(())
}
