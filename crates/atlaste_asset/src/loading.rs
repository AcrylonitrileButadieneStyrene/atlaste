use bevy::prelude::*;

#[derive(Component)]
pub struct ManagedAsset(pub UntypedHandle);

#[derive(EntityEvent)]
#[entity_event(auto_propagate)]
pub struct ManagedAssetLoaded {
    pub entity: Entity,
    pub handle: UntypedHandle,
    pub success: bool,
}

pub fn check(
    loading: Query<(Entity, &ManagedAsset)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, loading) in loading.iter() {
        let state = asset_server.load_state(&loading.0);
        if state.is_loading() {
            continue;
        }

        commands.trigger(ManagedAssetLoaded {
            entity,
            handle: loading.0.clone(),
            success: state.is_loaded(),
        });
        commands.entity(entity).despawn();
    }
}
