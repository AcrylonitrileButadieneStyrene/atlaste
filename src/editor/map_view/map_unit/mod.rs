use std::sync::Arc;

use atlaste_lcf::{MapUnitAsset, lcf::lmu::LcfMapUnit};
use bevy::prelude::*;

mod loading;
pub use loading::Loading;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, loading::check);
    }
}

#[derive(Component)]
pub struct MapUnit {
    #[allow(unused)] // keep the asset alive incase the map is loaded twice
    handle: Handle<MapUnitAsset>,
    pub map: Arc<LcfMapUnit>,
    #[allow(unused)]
    pub hash: u32, // todo: will be used for version control
}
