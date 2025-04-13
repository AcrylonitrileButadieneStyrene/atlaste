use bevy::prelude::*;

pub const FONT_SIZE: f32 = 12.;

use crate::{
    lcf_asset_loader::{GameAssets, MapTreeAsset},
    state::{CodePage, CurrentCodePage},
};

#[derive(Component)]
pub struct SearchMarker;

#[derive(Component)]
pub struct Entry(u16, Vec<u8>);

#[derive(Event)]
pub struct Redraw;

pub fn setup(
    mut commands: Commands,
    popups: Res<super::Popups>,
    game_data: Res<GameAssets>,
    map_trees: Res<Assets<MapTreeAsset>>,
    fonts: Res<crate::fonts::Fonts>,
) {
    let map_tree = map_trees.get(game_data.map_tree.id()).unwrap();

    let parent = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.),
            ..Default::default()
        })
        .set_parent(popups.map_tree.1)
        .id();

    commands
        .spawn((
            Node {
                border: UiRect::bottom(Val::Px(1.)),
                width: Val::Percent(100.),
                ..Default::default()
            },
            bevy_simple_text_input::TextInput,
            bevy_simple_text_input::TextInputSettings {
                retain_on_submit: true,
                ..Default::default()
            },
            bevy_simple_text_input::TextInputCursorPos::default(),
            bevy_simple_text_input::TextInputInactive(true),
            bevy_simple_text_input::TextInputPlaceholder {
                value: String::from("Search..."),
                text_font: Some(TextFont::from_font(fonts.default.clone())),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
            BorderColor(Color::hsl(0., 0., 0.4)),
            Button,
            SearchMarker,
        ))
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut query: Query<&mut bevy_simple_text_input::TextInputInactive>| {
                query.get_mut(trigger.entity()).unwrap().0 = false;
            },
        )
        .set_parent(parent);

    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                overflow: Overflow::scroll_y(),
                ..Default::default()
            },
            ScrollPosition::DEFAULT,
        ))
        .set_parent(parent)
        .id();

    recurse(&mut commands, &map_tree.0.maps, container, 0, 0);
    commands.trigger(Redraw);
}

pub fn destroy(mut commands: Commands, popups: Res<super::Popups>) {
    commands.entity(popups.map_tree.0).despawn_descendants();
}

fn recurse(
    commands: &mut Commands,
    map_tree: &[lcf_rs::lmt::Map],
    parent: Entity,
    current: usize,
    depth: usize,
) {
    let current = &map_tree[current];
    let id = current.id;

    commands
        .spawn((
            Node {
                margin: UiRect::left(Val::Px(depth as f32 * 4.0)),
                ..Default::default()
            },
            PickingBehavior {
                should_block_lower: false,
                ..Default::default()
            },
            Text::default(),
            TextFont::from_font_size(FONT_SIZE),
            Entry(id, current.name.clone()),
        ))
        .observe(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(crate::editor::map_view::Add(id));
        })
        .set_parent(parent);

    for child in &current.children {
        recurse(commands, map_tree, parent, *child as usize, depth + 1);
    }
}

pub fn redraw(
    _: Trigger<Redraw>,
    code_page: Res<CurrentCodePage>,
    mut names: Query<(&mut Text, &mut TextFont, &Entry)>,
    fonts: Res<crate::fonts::Fonts>,
) {
    let font = match code_page.0 {
        CodePage::ShiftJIS => fonts.jp.clone(),
        _ => fonts.default.clone(),
    };
    let encoding = code_page.0.to_encoding();

    names
        .par_iter_mut()
        .for_each(|(mut text, mut text_font, entry)| {
            // TODO: if the name doesn't include its map id, prepend it.
            text.0 = encoding.decode(&entry.1).0.to_string();
            text_font.font = font.clone();
        });
}

pub fn trigger(mut commands: Commands) {
    commands.trigger(Redraw);
}

pub fn search(
    mut events: EventReader<bevy_simple_text_input::TextInputSubmitEvent>,
    check: Query<Has<SearchMarker>>,
    mut query: Query<(&mut Node, &Text), With<Entry>>,
) {
    let event = events.read().last().unwrap();
    if check.get(event.entity).unwrap_or_default() {
        query.par_iter_mut().for_each(|(mut node, text)| {
            node.display = if text.0.contains(&event.value) {
                Display::DEFAULT
            } else {
                Display::None
            };
        });
    }
}
