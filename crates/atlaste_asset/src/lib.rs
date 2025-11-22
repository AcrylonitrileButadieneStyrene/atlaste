use bevy::prelude::*;

pub mod chipset;
mod image;
mod observed_asset;

pub use image::{
    dual::{DualR2kImage, DualR2kImageLoaded},
    r2k::{R2kImage, R2kImageLoadError},
};
pub use observed_asset::{ObservedAsset, ObservedAssetLoaded};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<R2kImage>()
            .register_asset_loader(image::r2k::R2kImageAssetLoader)
            .add_systems(Update, observed_asset::check)
            .add_observer(image::dual::on_add);
    }
}
