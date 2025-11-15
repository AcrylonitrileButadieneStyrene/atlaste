use bevy::{color::palettes::tailwind, prelude::*};

#[derive(Resource)]
pub struct Materials {
    pub deselected: Handle<ColorMaterial>,
    pub selected: Handle<ColorMaterial>,
}

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Materials {
        deselected: materials.add(ColorMaterial::from_color(tailwind::ZINC_600)),
        selected: materials.add(ColorMaterial::from_color(tailwind::YELLOW_500)),
    });
}

#[derive(Component)]
pub struct Selectable(pub Vec2, pub f32);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SelectionMarker;

#[derive(EntityEvent)]
struct Activation {
    entity: Entity,
    enabling: bool,
}

pub fn on_add(
    event: On<Add, Selectable>,
    query: Query<&Selectable>,
    mut commands: Commands,
    unit_rect: Res<crate::utils::unit_mesh::UnitRectangle>,
    outline: Res<Materials>,
) -> Result {
    let component = query.get(event.entity)?;
    let mut entity = commands.entity(event.entity);
    entity
        .observe(
            |event: On<Activation>,
             mut commands: Commands,
             children: Query<&Children>,
             mut visibility: Query<&mut Visibility, With<SelectionMarker>>|
             -> Result {
                match event.enabling {
                    true => commands.entity(event.entity).insert(Selected),
                    false => commands.entity(event.entity).remove::<Selected>(),
                };

                for child in children.get(event.entity)? {
                    let Ok(mut visibility) = visibility.get_mut(*child) else {
                        continue;
                    };

                    *visibility = match event.enabling {
                        true => Visibility::Inherited,
                        false => Visibility::Hidden,
                    };
                }

                Ok(())
            },
        )
        .with_children(|parent| {
            let base = Transform::from_scale(component.0.extend(1.0))
                .with_translation(Vec3::ZERO.with_z(component.1));
            parent.spawn((
                base,
                Mesh2d(unit_rect.0.clone()),
                MeshMaterial2d(outline.deselected.clone()),
            ));
            parent.spawn((
                SelectionMarker,
                base.with_translation(base.translation.with_z(base.translation.z + 0.1)),
                Visibility::Hidden,
                Mesh2d(unit_rect.0.clone()),
                MeshMaterial2d(outline.selected.clone()),
            ));
        });

    Ok(())
}

pub fn on_click(
    event: On<Pointer<Click>>,
    filter: Query<(), (With<Selectable>, Without<Selected>)>,
    mut commands: Commands,
    old: Query<Entity, With<Selected>>,
) {
    if event.button != PointerButton::Primary || filter.get(event.entity).is_err() {
        return;
    }

    for entity in old.iter() {
        commands.trigger(Activation {
            entity,
            enabling: false,
        });
    }

    commands.trigger(Activation {
        entity: event.entity,
        enabling: true,
    });
}

pub fn handle_keypress(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    selected: Query<Entity, With<Selected>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in selected.iter() {
            commands.trigger(Activation {
                entity,
                enabling: false,
            });
        }
    }
}
