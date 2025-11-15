use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d,
};

use crate::state::{CurrentCodePage, GameData};

pub fn on_spawn(
    setup: On<super::setup::Spawn>,
    mut commands: Commands,
    rectangle: Res<crate::utils::unit_mesh::UnitRectangle>,
    mut panorama_materials: ResMut<Assets<Material>>,
    code_page: Res<CurrentCodePage>,
    game: Res<GameData>,
    asset_server: Res<AssetServer>,
) {
    if !setup.map.panorama.enabled {
        return;
    }

    let Some(file) = &setup.map.panorama.file else {
        return;
    };

    commands.spawn((
        Name::new("Panorama"),
        ChildOf(setup.entity),
        Transform::from_scale(Vec3::new(
            setup.map.width as f32,
            setup.map.height as f32,
            1.0,
        )),
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
                let horizontal = convert_i32_to_b5(setup.map.panorama.horizontal_auto_scroll_speed);
                let vertical = convert_i32_to_b5(setup.map.panorama.vertical_auto_scroll_speed);

                u32::from_ne_bytes(
                    Options::new()
                        .with_width(setup.map.width as u16)
                        .with_height(setup.map.height as u16)
                        .with_horizontal(horizontal)
                        .with_vertical(vertical)
                        .into_bytes(),
                )
            },
        })),
    ));
}

#[derive(Clone, Debug, Asset, AsBindGroup, TypePath)]
pub struct Material {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub options: u32,
}

impl Material2d for Material {
    fn fragment_shader() -> ShaderRef {
        "shaders/panorama.wgsl".into()
    }
}

#[modular_bitfield::bitfield]
struct Options {
    #[skip(getters)]
    width: modular_bitfield::specifiers::B9,
    #[skip(getters)]
    height: modular_bitfield::specifiers::B9,
    #[skip(getters)]
    horizontal: modular_bitfield::specifiers::B5,
    #[skip(getters)]
    vertical: modular_bitfield::specifiers::B5,
    #[skip(getters)]
    disable: modular_bitfield::specifiers::B1,
    #[skip]
    __padding: modular_bitfield::specifiers::B3,
}

const fn convert_i32_to_b5(val: i32) -> u8 {
    debug_assert!(val >= -15, "too small, i5::MIN is -15");
    debug_assert!(val <= 16, "too big, i5::MAX is 16");

    (val + 15) as u8 % 31
}
