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
    let DualR2kImage { base, file } = query.get(event.entity)?;
    let png = base.resolve(&format!("{}.png", file)).unwrap();

    commands.spawn((
        ChildOf(event.entity),
        crate::ObservedAsset {
            handle: asset_server.load::<crate::R2kImage>(png).untyped(),
            despawn: true,
        },
        observe(on_png_loaded),
    ));

    Ok(())
}

fn on_png_loaded(
    event: On<crate::ObservedAssetLoaded>,
    parent: Query<&ChildOf>,
    query: Query<&DualR2kImage>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> Result {
    let parent = parent.get(event.entity)?.0;
    match event.status {
        Ok(()) => {
            commands.trigger(DualR2kImageLoaded {
                entity: parent,
                handle: event.handle.clone().typed(),
                status: Ok(()),
            });
        }
        Err(_) => {
            let DualR2kImage { base, file } = query.get(parent)?;
            let bmp = base.resolve(&format!("{}.bmp", file)).unwrap();

            commands.spawn((
                ChildOf(parent),
                crate::ObservedAsset {
                    handle: asset_server.load::<crate::R2kImage>(&bmp).untyped(),
                    despawn: true,
                },
                observe(on_bmp_loaded),
            ));
        }
    }

    Ok(())
}

fn on_bmp_loaded(
    event: On<crate::ObservedAssetLoaded>,
    parent: Query<&ChildOf>,
    mut commands: Commands,
) -> Result {
    let parent = parent.get(event.entity)?.0;
    commands.trigger(DualR2kImageLoaded {
        entity: parent,
        handle: event.handle.clone().typed(),
        status: event.status.clone(),
    });

    Ok(())
}
