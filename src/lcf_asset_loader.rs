use bevy::{asset::AssetLoader, prelude::*};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DataBaseAsset>()
            .init_asset::<MapTreeAsset>()
            .init_asset::<MapUnitAsset>()
            .init_asset_loader::<DataBaseLoader>()
            .init_asset_loader::<MapTreeLoader>()
            .init_asset_loader::<MapUnitLoader>()
            .add_systems(
                Startup,
                retrigger_load.run_if(resource_exists::<crate::app::GameDir>),
            )
            .add_observer(load_game)
            .add_observer(on_asset_load);
    }
}

#[derive(Event, Resource)]
pub struct DataBaseHandle(pub Handle<DataBaseAsset>);

#[derive(Event, Resource)]
pub struct MapTreeHandle(pub Handle<MapTreeAsset>);

macro_rules! loader {
    ($loader:ident, $asset:ident, $type:ty, ($exts:expr)) => {
        #[derive(Default)]
        pub struct $loader;
        impl AssetLoader for $loader {
            type Asset = $asset;
            type Settings = ();
            type Error = LcfAssetLoaderError;

            async fn load(
                &self,
                reader: &mut dyn bevy::asset::io::Reader,
                _settings: &Self::Settings,
                _load_context: &mut bevy::asset::LoadContext<'_>,
            ) -> Result<Self::Asset, Self::Error> {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf).await?;
                match <$type>::from_bytes(&buf) {
                    Ok((_, Ok(x))) => Ok($asset(x)),
                    Ok((_, Err(err))) => Err(LcfAssetLoaderError::Build(err)),
                    Err(err) => Err(LcfAssetLoaderError::Parse(err.to_owned())),
                }
            }

            fn extensions(&self) -> &[&str] {
                &[$exts]
            }
        }
    };
}

#[derive(Asset, Debug, TypePath)]
pub struct DataBaseAsset(pub lcf_rs::LcfDataBase);
loader!(DataBaseLoader, DataBaseAsset, lcf_rs::LcfDataBase, ("ldb"));

#[derive(Asset, Debug, TypePath)]
pub struct MapTreeAsset(pub lcf_rs::LcfMapTree);
loader!(MapTreeLoader, MapTreeAsset, lcf_rs::LcfMapTree, ("lmt"));

#[derive(Asset, Debug, TypePath)]
pub struct MapUnitAsset(pub lcf_rs::LcfMapUnit);
loader!(MapUnitLoader, MapUnitAsset, lcf_rs::LcfMapUnit, ("lmu"));

#[derive(Debug, thiserror::Error)]
pub enum LcfAssetLoaderError {
    #[error("IO error: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0:?}")]
    Parse(#[from] lcf_rs::nom::Err<lcf_rs::nom::error::Error<Vec<u8>>>),
    #[error("Build error: {0:?}")]
    Build(#[from] lcf_rs::Error),
}

pub fn retrigger_load(mut commands: Commands, game_dir: Res<crate::app::GameDir>) {
    commands.trigger(game_dir.clone());
}

pub fn load_game(
    trigger: Trigger<crate::app::GameDir>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(DataBaseHandle(
        asset_server.load(trigger.0.join("RPG_RT.ldb")),
    ));
    commands.insert_resource(MapTreeHandle(
        asset_server.load(trigger.0.join("RPG_RT.lmt")),
    ));
}

pub fn on_asset_load(trigger: Trigger<AssetEvent<DataBaseAsset>>) {
    dbg!(trigger);
}
