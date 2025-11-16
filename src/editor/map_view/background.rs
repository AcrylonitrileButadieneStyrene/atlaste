use bevy::{color::palettes::basic::BLACK, prelude::*};

use crate::editor::map_view::map_unit::MapUnit;

pub fn on_add_map_unit(
    event: On<Add, MapUnit>,
    query: Query<&MapUnit>,
    mut commands: Commands,
    rectangle: Res<crate::utils::unit_mesh::UnitRectangle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result {
    let MapUnit { map, .. } = query.get(event.entity)?;

    commands.spawn((
        Name::new("Background"),
        ChildOf(event.entity),
        Transform::from_translation(-Vec3::Z),
        Visibility::Inherited,
        crate::editor::select::Selectable(
            Vec2::new(map.width as f32 + 0.2, map.height as f32 + 0.2),
            -2.0,
        ),
        Children::spawn((
            // prevent transparency from seeing the window outline
            Spawn((
                Name::new("Cover"),
                Transform::from_translation(Vec3::new(0., 0., -1.)).with_scale(Vec3::new(
                    map.width as f32,
                    map.height as f32,
                    1.0,
                )),
                Pickable::IGNORE,
                Mesh2d(rectangle.0.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLACK))),
            )),
        )),
    ));

    Ok(())
}
