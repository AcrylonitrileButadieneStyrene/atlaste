use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::state::CodePage;
use crate::{
    lcf_asset_loader::{MapTreeAsset, MapTreeHandle},
    state::Encoding,
};

#[derive(Resource)]
pub struct SelectedMap(pub u16);

#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Entry(u16, Vec<u8>);

#[derive(Event)]
pub struct Redraw;

pub fn setup(
    mut commands: Commands,
    game_data: Res<MapTreeHandle>,
    map_trees: Res<Assets<MapTreeAsset>>,
) {
    let map_tree = map_trees
        .get(game_data.0.id())
        .expect("map tree loaded in time");

    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(Val::Px(-24.0)),
                height: Val::Px(24.0 * map_tree.0.maps.len() as f32),
                ..Default::default()
            },
            Marker,
        ))
        .id();
    recurse(&mut commands, &map_tree.0.maps, container, 0, 0, 0);

    commands.trigger(Redraw);
}

fn recurse(
    commands: &mut Commands,
    map_tree: &[lcf_rs::lmt::Map],
    parent: Entity,
    current: usize,
    depth: usize,
    sibling: usize,
) {
    let current = &map_tree[current];
    let node = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(Val::Px(if sibling == 0 { 24.0 } else { 0.0 }))
                    .with_left(Val::Px(depth as f32 * 8.0)),
                ..Default::default()
            },
            Text::default(),
            TextFont::default(),
            Entry(current.own, current.name.clone()),
        ))
        .set_parent(parent)
        .id();
    for (index, child) in current.children.iter().enumerate() {
        recurse(commands, map_tree, node, *child as usize, depth + 1, index);
    }
}

pub fn redraw(
    _: Trigger<Redraw>,
    code_page: Res<CodePage>,
    mut names: Query<(&mut Text, &mut TextFont, &Entry)>,
    fonts: Res<crate::fonts::Fonts>,
) {
    let font = match code_page.0 {
        Encoding::ShiftJIS => fonts.jp.clone(),
        _ => fonts.default.clone(),
    };
    let encoding = code_page.0.to_encoding();

    names
        .par_iter_mut()
        .for_each(|(mut text, mut text_font, entry)| {
            text.0 = encoding.decode(&entry.1).0.to_string();
            text_font.font = font.clone();
        });
}

pub fn click(mut commands: Commands, entries: Query<(&Interaction, &Entry)>) {
    for (interaction, entry) in entries.iter() {
        if matches!(interaction, Interaction::Pressed) {
            commands.insert_resource(SelectedMap(entry.0));
        }
    }
}

pub fn scroll(mut wheel: EventReader<MouseWheel>, mut node: Query<&mut Node, With<Marker>>) {
    let mut node = node.single_mut();
    for event in wheel.read() {
        let dy = match event.unit {
            MouseScrollUnit::Line => event.y * 24.,
            MouseScrollUnit::Pixel => event.y,
        };

        let unwrap = |x| if let Val::Px(y) = x { y } else { 0.0 };
        let offset = unwrap(node.top);
        let height = unwrap(node.height);

        // This will panic if there are 0 maps. Oh well.
        node.top = Val::Px((offset + dy).clamp(-height + 24.0, 0.0));
    }
}

pub fn destroy(mut commands: Commands, marked: Query<Entity, With<Marker>>) {
    for marked in marked.iter() {
        commands.entity(marked).despawn_recursive();
    }
}
