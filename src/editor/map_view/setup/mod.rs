use std::sync::Arc;

use atlaste_lcf::lcf::lmu::LcfMapUnit;
use bevy::{
    color::palettes::basic::BLACK,
    prelude::*,
};

pub mod tiles;

#[derive(EntityEvent)]
pub struct Spawn {
    pub entity: Entity,
    pub map: Arc<LcfMapUnit>,
    pub chipset: Handle<Image>,
}

pub fn on_spawn_background(
    setup: On<Spawn>,
    mut commands: Commands,
    rectangle: Res<crate::utils::unit_mesh::UnitRectangle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Background"),
        ChildOf(setup.entity),
        Transform::from_translation(-Vec3::Z),
        Visibility::Inherited,
        crate::editor::select::Selectable(
            Vec2::new(setup.map.width as f32 + 0.2, setup.map.height as f32 + 0.2),
            -2.0,
        ),
        Children::spawn((
            // prevent transparency from seeing the window outline
            Spawn((
                Name::new("Cover"),
                Transform::from_translation(Vec3::new(0., 0., -1.)).with_scale(Vec3::new(
                    setup.map.width as f32,
                    setup.map.height as f32,
                    1.0,
                )),
                Pickable::IGNORE,
                Mesh2d(rectangle.0.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(BLACK))),
            )),
        )),
    ));
}
