use std::sync::Arc;

use bevy::{
    asset::{AssetLoadError, AssetPath},
    prelude::*,
    ui_widgets::observe,
};

#[derive(Component)]
pub struct DualR2kImage {
    pub base: AssetPath<'static>,
    pub file: String,
}

#[derive(EntityEvent)]
pub struct DualR2kImageLoaded {
    pub entity: Entity,
    pub handle: Handle<crate::R2kImage>,
    pub status: Result<(), Option<Arc<AssetLoadError>>>,
}

pub fn on_add(
    event: On<Add, DualR2kImage>,
    query: Query<&DualR2kImage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) -> Result {
    let entity = event.entity;
    let dual = query.get(entity)?;
    let png = dual.base.resolve(&format!("{}.png", dual.file)).unwrap();
    let bmp = dual.base.resolve(&format!("{}.bmp", dual.file)).unwrap();

    commands.spawn((
        ChildOf(entity),
        crate::ObservedAsset {
            handle: asset_server.load::<crate::R2kImage>(png).untyped(),
            despawn: true,
        },
        observe(
            move |event: On<crate::ObservedAssetLoaded>,
                  mut commands: Commands,
                  asset_server: Res<AssetServer>| match event.status {
                Ok(()) => {
                    commands.trigger(DualR2kImageLoaded {
                        entity,
                        handle: event.handle.clone().typed(),
                        status: Ok(()),
                    });
                }
                Err(_) => {
                    commands.spawn((
                        ChildOf(entity),
                        crate::ObservedAsset {
                            handle: asset_server.load::<crate::R2kImage>(&bmp).untyped(),
                            despawn: true,
                        },
                        observe(
                            move |event: On<crate::ObservedAssetLoaded>, mut commands: Commands| {
                                commands.trigger(DualR2kImageLoaded {
                                    entity,
                                    handle: event.handle.clone().typed(),
                                    status: event.status.clone(),
                                });
                            },
                        ),
                    ));
                }
            },
        ),
    ));

    Ok(())
}
