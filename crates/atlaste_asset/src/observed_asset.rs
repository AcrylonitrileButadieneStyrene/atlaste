use std::sync::Arc;

use bevy::{
    asset::{AssetLoadError, DependencyLoadState, LoadState, RecursiveDependencyLoadState},
    prelude::*,
};

#[derive(Component)]
pub struct ObservedAsset {
    pub handle: UntypedHandle,
    /// Despawn the entity the component is attached to when loading stops.
    pub despawn: bool,
}

#[derive(EntityEvent)]
pub struct ObservedAssetLoaded {
    pub entity: Entity,
    pub handle: UntypedHandle,
    pub status: Result<(), Option<Arc<AssetLoadError>>>,
}

pub fn check(
    loading: Query<(Entity, &ObservedAsset)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, loading) in loading.iter() {
        let state = flatten(asset_server.get_load_states(&loading.handle));
        if state.is_loading() {
            continue;
        }

        commands.trigger(ObservedAssetLoaded {
            entity,
            handle: loading.handle.clone(),
            status: match state {
                bevy::asset::LoadState::Loaded => Ok(()),
                bevy::asset::LoadState::Failed(err) => Err(Some(err.clone())),
                bevy::asset::LoadState::NotLoaded => Err(None),
                bevy::asset::LoadState::Loading => unreachable!(),
            },
        });

        if loading.despawn {
            commands.entity(entity).despawn();
        } else {
            commands.entity(entity).remove::<ObservedAsset>();
        }
    }
}

fn flatten(
    states: Option<(LoadState, DependencyLoadState, RecursiveDependencyLoadState)>,
) -> LoadState {
    match states.unwrap_or((
        LoadState::NotLoaded,
        DependencyLoadState::NotLoaded,
        RecursiveDependencyLoadState::NotLoaded,
    )) {
        // the root asset is not loading
        (LoadState::NotLoaded, _, _) => LoadState::NotLoaded,

        // any errors have occurred
        (LoadState::Failed(err), _, _)
        | (_, DependencyLoadState::Failed(err), _)
        | (_, _, RecursiveDependencyLoadState::Failed(err)) => LoadState::Failed(err),

        // everything has finished loading
        // todo: determine if assets without dependencies are loaded or not loaded when finished
        (
            LoadState::Loaded,
            DependencyLoadState::Loaded | DependencyLoadState::NotLoaded,
            RecursiveDependencyLoadState::Loaded | RecursiveDependencyLoadState::NotLoaded,
        ) => LoadState::Loaded,

        // anything is still loaded and nothing is errored
        (LoadState::Loading, _, _)
        | (_, DependencyLoadState::Loading, _)
        | (_, _, RecursiveDependencyLoadState::Loading) => LoadState::Loading,
    }
}
