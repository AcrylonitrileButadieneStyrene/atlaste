use atlaste_ui::sections::{
    layers::{Layer, Selected},
    tools::Tool,
};
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
pub struct CreateSelectable(pub Vec2, pub f32);

#[derive(Component)]
#[relationship_target(relationship = SelectionIndicator, linked_spawn)]
pub struct Selectable(Entity);

#[derive(Component)]
#[relationship(relationship_target = Selectable)]
pub struct SelectionIndicator(Entity);

pub fn on_add(
    event: On<Add, CreateSelectable>,
    query: Query<&CreateSelectable, With<Layer>>,
    mut commands: Commands,
    unit_rect: Res<crate::utils::unit_mesh::UnitRectangle>,
    outline: Res<Materials>,
) -> Result {
    let create = query.get(event.entity)?;

    commands
        .entity(event.entity)
        .remove::<CreateSelectable>()
        .observe(
            |click: On<Pointer<Click>>,
             item: Query<&Layer>,
             mut commands: Commands,
             state: Res<State<Tool>>|
             -> Result {
                if *state == Tool::Select {
                    let mut event = click.clone();
                    event.entity = *item.get(click.entity)?.collection();
                    commands.trigger(event);
                };

                Ok(())
            },
        )
        .with_children(|parent| {
            let base = Transform::from_scale(create.0.extend(1.0))
                .with_translation(Vec3::ZERO.with_z(create.1));
            // always visible, looks deselected, gets covered by the selection indicator
            parent.spawn((
                base,
                Mesh2d(unit_rect.0.clone()),
                MeshMaterial2d(outline.deselected.clone()),
            ));
            parent.spawn((
                SelectionIndicator(event.entity),
                base.with_translation(base.translation.with_z(base.translation.z + 0.1)),
                Visibility::Hidden,
                Mesh2d(unit_rect.0.clone()),
                MeshMaterial2d(outline.selected.clone()),
            ));
        });

    Ok(())
}

pub fn on_select(
    event: On<Add, Selected>,
    query: Query<&Selectable>,
    visibility: Query<&mut Visibility>,
) {
    set(event.entity, query, visibility, Visibility::Inherited);
}

pub fn on_deselect(
    event: On<Remove, Selected>,
    query: Query<&Selectable>,
    visibility: Query<&mut Visibility>,
) {
    set(event.entity, query, visibility, Visibility::Hidden);
}

fn set(
    entity: Entity,
    query: Query<&Selectable>,
    mut visibility: Query<&mut Visibility>,
    value: Visibility,
) {
    if let Ok(selectable) = query.get(entity)
        && let Ok(mut visibility) = visibility.get_mut(*selectable.collection())
    {
        *visibility = value;
    }
}
