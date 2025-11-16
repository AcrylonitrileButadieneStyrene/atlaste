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
                        .join("Panorama/")
                        .join(code_page.0.to_encoding().decode(file).0.to_string())
                        .with_added_extension("png") // todo: this one can also be a .bmp
                }),
                options: {
                    let horizontal = convert_i32_to_b5(map.panorama.horizontal_auto_scroll_speed);
                    let vertical = convert_i32_to_b5(map.panorama.vertical_auto_scroll_speed);
                    u32::from_ne_bytes(
                        Options::new()
                            .with_width(map.width as u16)
                            .with_height(map.height as u16)
                            .with_horizontal(horizontal)
                            .with_vertical(vertical)
                            .into_bytes(),
                    )
                },
            })),
        ));
    }

    Ok(())
}

const fn convert_i32_to_b5(val: i32) -> u8 {
    debug_assert!(val >= -15, "too small, i5::MIN is -15");
    debug_assert!(val <= 16, "too big, i5::MAX is 16");

    (val + 15) as u8 % 31
}
