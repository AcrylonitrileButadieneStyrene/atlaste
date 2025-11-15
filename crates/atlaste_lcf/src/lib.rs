use std::{path::PathBuf, sync::Arc};

use bevy::prelude::*;

mod asset_loader;
pub use asset_loader::*;
use lcf::{ldb::LcfDataBase, lmt::LcfMapTree};

pub use lcf;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<IsSomethingLoadingState>()
            .init_asset::<DataBaseAsset>()
            .init_asset::<MapTreeAsset>()
            .init_asset::<MapUnitAsset>()
            .init_asset_loader::<DataBaseLoader>()
            .init_asset_loader::<MapTreeLoader>()
            .init_asset_loader::<MapUnitLoader>()
            .add_systems(
                Update,
                check_asset.run_if(in_state(IsSomethingLoadingState::Yes)),
            )
            .add_observer(on_load);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
enum IsSomethingLoadingState {
    #[default]
    No,
    Yes,
}

#[derive(Debug, Event)]
pub struct Load(pub PathBuf);

#[derive(Debug, Event)]
pub struct Loaded(pub Arc<Game>);

#[derive(Clone, Debug)]
pub struct Game {
    pub database: Arc<LcfDataBase>,
    pub database_hash: u32,
    pub map_tree: Arc<LcfMapTree>,
    pub map_tree_hash: u32,
    pub game_dir: PathBuf,
}

#[derive(Component)]
struct Loading {
    database: Handle<DataBaseAsset>,
    map_tree: Handle<MapTreeAsset>,
    path: PathBuf,
}

fn on_load(
    load: On<Load>,
    asset_server: Res<AssetServer>,
    mut next: ResMut<NextState<IsSomethingLoadingState>>,
    mut commands: Commands,
) {
    info!("Load started");
    let path = load.0.clone();
    let database = asset_server.load::<DataBaseAsset>(path.join("RPG_RT.ldb"));
    let map_tree = asset_server.load::<MapTreeAsset>(path.join("RPG_RT.lmt"));
    commands.spawn(Loading {
        database,
        map_tree,
        path,
    });
    next.set(IsSomethingLoadingState::Yes);
}

fn check_asset(
    mut query: Query<(Entity, &Loading)>,
    asset_server: Res<AssetServer>,
    mut databases: ResMut<Assets<DataBaseAsset>>,
    mut map_trees: ResMut<Assets<MapTreeAsset>>,
    mut commands: Commands,
    mut next: ResMut<NextState<IsSomethingLoadingState>>,
) {
    for (ent, loading) in query.iter_mut() {
        let database_loading = asset_server
            .get_load_state(&loading.database)
            .unwrap()
            .is_loading();
        let map_tree_loading = asset_server
            .get_load_state(&loading.map_tree)
            .unwrap()
            .is_loading();
        if !database_loading && !map_tree_loading {
            let database = databases.remove(&loading.database).unwrap();
            let map_tree = map_trees.remove(&loading.map_tree).unwrap();
            info!("Load completed");
            commands.entity(ent).despawn();
            commands.trigger(Loaded(Arc::new(Game {
                database: database.data.clone(),
                database_hash: database.hash,
                map_tree: map_tree.data.clone(),
                map_tree_hash: map_tree.hash,
                game_dir: loading.path.clone(),
            })));
        }
    }

    if query.is_empty() {
        next.set(IsSomethingLoadingState::No);
    }
}
