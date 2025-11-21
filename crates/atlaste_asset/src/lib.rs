use bevy::prelude::*;

pub mod chipset;
mod loading;
mod r2k_image;

pub use loading::{ManagedAsset, ManagedAssetLoaded};
pub use r2k_image::{R2kImage, R2kImageLoadError};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<R2kImage>()
            .register_asset_loader(r2k_image::R2kImageAssetLoader)
            .add_systems(Update, loading::check);
    }
}
