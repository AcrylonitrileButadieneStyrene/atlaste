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
        "shaders/panorama.wgsl".into()
    }
}

#[modular_bitfield::bitfield]
pub struct Options {
    #[skip(getters)]
    pub width: modular_bitfield::specifiers::B9,
    #[skip(getters)]
    pub height: modular_bitfield::specifiers::B9,
    #[skip(getters)]
    pub horizontal: modular_bitfield::specifiers::B5,
    #[skip(getters)]
    pub vertical: modular_bitfield::specifiers::B5,
    #[skip(getters)]
    pub disable: modular_bitfield::specifiers::B1,
    #[skip]
    __padding: modular_bitfield::specifiers::B3,
}
