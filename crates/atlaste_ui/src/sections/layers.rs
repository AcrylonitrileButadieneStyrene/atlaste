use bevy::{feathers::theme::ThemeBackgroundColor, prelude::*, ui_widgets::observe};

#[derive(Component)]
pub struct CreateLayer {
    pub name: String,
}

#[derive(Component)]
#[relationship_target(relationship = LayerListItem, linked_spawn)]
pub struct Layer(Entity);

#[derive(Component)]
#[relationship(relationship_target = Layer)]
pub struct LayerListItem(Entity);

#[derive(Component)]
pub struct LayerListMarker;

#[derive(Clone, Component)]
pub struct Selected;

#[derive(Event)]
pub struct DeselectAll;

pub fn new() -> impl Bundle {
    (
        LayerListMarker,
        Node {
            flex_direction: FlexDirection::Column,
            grid_row: GridPlacement::start(3),
            grid_column: GridPlacement::start(4),
            padding: UiRect::all(px(4)),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
    )
}

pub fn new_item(text: String) -> impl Bundle {
    (Text::new(text), Button, observe(on_click))
}

pub fn on_create_add(
    event: On<Add, CreateLayer>,
    query: Query<&CreateLayer>,
    list: Query<Entity, With<LayerListMarker>>,
    mut commands: Commands,
) -> Result {
    let list = list.single()?;

    commands.entity(event.entity).remove::<CreateLayer>();
    commands.spawn((
        LayerListItem(event.entity),
        ChildOf(list),
        new_item(query.get(event.entity)?.name.clone()),
    ));

    Ok(())
}

fn on_click(
    event: On<Pointer<Click>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    layer_item: Query<&LayerListItem>,
) -> Result {
    if !keys.pressed(KeyCode::ControlLeft) {
        commands.trigger(DeselectAll);
    }

    commands
        .entity(layer_item.get(event.entity)?.0)
        .insert_recursive::<Layer>(Selected);
    Ok(())
}

pub fn on_deselect_all(
    _: On<DeselectAll>,
    selected: Query<Entity, (With<Layer>, With<Selected>)>,
    mut commands: Commands,
) -> Result {
    for entity in selected {
        commands
            .entity(entity)
            .remove_recursive::<Layer, Selected>();
    }

    Ok(())
}

pub fn handle_escape(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.trigger(DeselectAll);
    }
}
