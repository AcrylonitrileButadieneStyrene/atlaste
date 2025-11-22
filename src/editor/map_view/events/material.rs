use atlaste_lcf::lcf::{enums::AnimationType, lmu::event::page::Graphic};
use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d,
};

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
        "shaders/charset.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        bevy::sprite_render::AlphaMode2d::Blend
    }
}

#[modular_bitfield::bitfield]
pub struct Options {
    #[skip(getters)]
    pub index: modular_bitfield::specifiers::B3,
    #[skip(getters)]
    pub pattern: modular_bitfield::specifiers::B2,
    #[skip(getters)]
    pub direction: modular_bitfield::specifiers::B2,
    #[skip(getters)]
    pub animation: modular_bitfield::specifiers::B3,
    #[skip]
    __padding: modular_bitfield::specifiers::B22,
}

impl Options {
    pub fn from_event(graphic: &Graphic, animation: AnimationType) -> Self {
        Self::new()
            .with_index(graphic.index as u8)
            .with_pattern(graphic.pattern as u8)
            .with_direction(graphic.direction as u8)
            .with_animation(animation as u8)
    }
}
