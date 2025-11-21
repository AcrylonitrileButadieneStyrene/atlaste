use bevy::{prelude::*, sprite_render::Material2dPlugin};

use crate::{
    editor::map_view::map_unit::MapUnit,
    state::{CurrentCodePage, GameData},
    utils::unit_mesh::UnitRectangle,
};

mod charset;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<charset::Material>::default())
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
    mut materials: ResMut<Assets<charset::Material>>,
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

        dbg!(event.id);
        commands.spawn((
            ChildOf(container),
            Transform::from_scale(Vec3::new(1.5, 2., 1.)).with_translation(Vec3::new(
                event.x as f32 - map.width as f32 / 2.0 + 0.5,
                (map.height - event.y) as f32 - map.height as f32 / 2.0,
                (event.y + event.x) as f32 / 1000.,
            )),
            Mesh2d(rectange.0.clone()),
            MeshMaterial2d(
                materials.add(charset::Material {
                    texture: asset_server.load(
                        game.game_dir
                            .resolve(&format!(
                                "CharSet/{}.png", // todo: also this one can also be a .bmp
                                dbg!(
                                    codepage
                                        .0
                                        .to_encoding()
                                        .decode(&page.graphic.file)
                                        .0
                                        .to_string()
                                )
                            ))
                            .unwrap(),
                    ),
                    options: dbg!(u32::from_ne_bytes(
                        charset::Options::from_event(&page.graphic, &page.animation_type)
                            .into_bytes(),
                    )),
                }),
            ),
        ));
    }

    Ok(())
}
