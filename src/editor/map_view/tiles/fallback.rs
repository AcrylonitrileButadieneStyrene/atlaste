use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Resource)]
pub struct Fallback(pub Handle<Image>);

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(Fallback(images.add(Image::new_fill(
        Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 480,
        },
        TextureDimension::D2,
        &[0x00, 0xFF, 0x00, 0x00],
        TextureFormat::bevy_default(),
        RenderAssetUsages::RENDER_WORLD,
    ))));
}
