pub mod settings;
pub mod toolbar;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Slots {
    pub toolbar: Entity,
    pub settings: Entity,
    pub content: Entity,
    pub properties: Entity,
    pub logs: Entity,
}

pub fn setup(mut commands: Commands) {
    let background = BackgroundColor(Color::hsl(0.0, 0.0, 0.1));
    let border = BorderColor(Color::hsl(0.0, 0.0, 0.4));

    let root = commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ZIndex(-1),
            PickingBehavior::IGNORE,
        ))
        .id();

    let toolbar = commands
        .spawn((
            Node {
                border: UiRect::bottom(Val::Px(1.0)),
                flex_direction: FlexDirection::Row,
                height: Val::Px(32.0),
                ..Default::default()
            },
            background,
            border,
        ))
        .set_parent(root)
        .id();

    let horizontal = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                ..Default::default()
            },
            PickingBehavior::IGNORE,
        ))
        .set_parent(root)
        .id();

    let settings = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                width: Val::Px(200.0),
                ..Default::default()
            },
            background,
            border,
        ))
        .set_parent(horizontal)
        .id();

    let content = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                ..Default::default()
            },
            PickingBehavior::IGNORE,
        ))
        .set_parent(horizontal)
        .id();

    let properties = commands
        .spawn((
            Node {
                border: UiRect::left(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                width: Val::Px(200.0),
                ..Default::default()
            },
            background,
            border,
        ))
        .set_parent(horizontal)
        .id();

    let logs = commands
        .spawn((
            Node {
                border: UiRect::top(Val::Px(1.0)),
                flex_direction: FlexDirection::Row,
                height: Val::Px(32.0),
                ..Default::default()
            },
            background,
            border,
        ))
        .set_parent(root)
        .id();

    commands.insert_resource(Slots {
        toolbar,
        settings,
        content,
        properties,
        logs,
    });
}
