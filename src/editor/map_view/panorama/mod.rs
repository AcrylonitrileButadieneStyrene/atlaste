use atlaste_lcf::lcf::lmu::PanoramaOptions;
use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
    utils::unit_mesh::UnitRectangle,
};

mod material;

pub use material::{Material, Options};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Material>::default())
            .add_observer(on_spawn);
    }
}

fn on_spawn(
    setup: On<Add, MapUnit>,
    query: Query<&MapUnit>,
    mut commands: Commands,
    rectangle: Res<UnitRectangle>,
    mut panorama_materials: ResMut<Assets<Material>>,
    code_page: Res<CurrentCodePage>,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) -> Result {
    let MapUnit { map, .. } = query.get(setup.entity)?;

    if map.panorama.enabled
        && let Some(file) = &map.panorama.file
    {
        commands.spawn((
            Name::new("Panorama"),
            ChildOf(setup.entity),
            Transform::from_scale(Vec3::new(map.width as f32, map.height as f32, 1.0)),
            Pickable::IGNORE,
            Mesh2d(rectangle.0.clone()),
            MeshMaterial2d(panorama_materials.add(Material {
                texture: asset_server.load({
                    game.game_dir
                        .resolve(&format!(
                            "Panorama/{}.png", // todo: this one can also be a .bmp
                            code_page.0.to_encoding().decode(file).0
                        ))
                        .unwrap()
                }),
                options: {
                    let convert = |opt: &PanoramaOptions| {
                        (match opt {
                            PanoramaOptions::NoLoop | PanoramaOptions::NoAutoscroll => 0,
                            PanoramaOptions::Autoscroll(x) => *x,
                        } + 15) as u8
                            % 31
                    };

                    u32::from_ne_bytes(
                        Options::new()
                            .with_width(map.width as u16)
                            .with_height(map.height as u16)
                            .with_horizontal(convert(&map.panorama.horizontal))
                            .with_vertical(convert(&map.panorama.vertical))
                            .into_bytes(),
                    )
                },
            })),
        ));
    }

    Ok(())
}
