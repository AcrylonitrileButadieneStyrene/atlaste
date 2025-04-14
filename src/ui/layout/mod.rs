mod settings;
mod toolbar;
mod tools;

use crate::state::CurrentCodePage;
use bevy::prelude::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            ((setup, (toolbar::setup, settings::setup, tools::setup)).chain(),),
        )
        .add_systems(
            Update,
            settings::highlight_codepage.run_if(resource_exists_and_changed::<CurrentCodePage>),
        );
    }
}

#[derive(Resource)]
pub struct Slots {
    pub top: Entity,
    pub left: Entity,
    pub center: Entity,
    pub right: Entity,
    pub bottom: Entity,
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

    let top = commands
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

    let left = commands
        .spawn((
            Node {
                border: UiRect::right(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background,
            border,
        ))
        .set_parent(horizontal)
        .id();

    let center = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                ..Default::default()
            },
            PickingBehavior::IGNORE,
        ))
        .set_parent(horizontal)
        .id();

    let right = commands
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

    let bottom = commands
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
        top,
        left,
        center,
        right,
        bottom,
    });
}
