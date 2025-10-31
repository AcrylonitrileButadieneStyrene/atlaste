use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputCursorPos, TextInputInactive, TextInputPlaceholder, TextInputSettings,
    TextInputSubmitMessage,
};

use crate::state::{CodePage, CurrentCodePage, GameData};

#[derive(Component)]
pub struct SearchMarker;

#[derive(Component)]
pub struct Entry(u32, Vec<u8>);

#[derive(Event)]
pub struct UpdateText;

pub fn setup(
    mut commands: Commands,
    layout: Res<super::Layout>,
    game_data: Res<GameData>,
    fonts: Res<crate::fonts::Fonts>,
) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(1),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
            Pickable::IGNORE,
            ChildOf(layout.root),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        border: UiRect::bottom(Val::Px(1.)),
                        width: Val::Percent(100.),
                        ..Default::default()
                    },
                    TextInput,
                    TextInputSettings {
                        retain_on_submit: true,
                        ..Default::default()
                    },
                    TextInputCursorPos::default(),
                    TextInputInactive(true),
                    TextInputPlaceholder {
                        value: String::from("Search..."),
                        text_font: Some(TextFont::default().with_font(fonts.default.clone())),
                        ..Default::default()
                    },
                    SearchMarker,
                ))
                .observe(
                    |trigger: On<Pointer<Click>>, mut query: Query<&mut TextInputInactive>| {
                        query.get_mut(trigger.entity).unwrap().0 = false;
                    },
                );

            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollPosition::DEFAULT,
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    for (id, map) in &game_data.map_tree.maps {
                        let id = *id;

                        parent.spawn((
                            Node {
                                margin: UiRect::left(Val::Px(map.indentation as f32 * 8.0)),
                                ..Default::default()
                            },
                            Pickable {
                                should_block_lower: false,
                                ..Default::default()
                            },
                            Text::default(),
                            TextFont::from_font_size(12.),
                            Entry(id, map.name.clone()),
                        ));
                    }
                })
                .observe(
                    move |click: On<Pointer<Click>>,
                          entry: Query<&Entry>,
                          mut commands: Commands| {
                        if let Ok(entry) = entry.get(click.trigger().original_event_target) {
                            commands.trigger(crate::editor::map_view::Add(entry.0));
                        }
                    },
                );
        });

    commands.trigger(UpdateText);
}

pub fn update_text(
    _: On<UpdateText>,
    code_page: Res<CurrentCodePage>,
    fonts: Res<crate::fonts::Fonts>,
    mut names: Query<(&mut Text, &mut TextFont, &Entry)>,
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

pub fn search(
    mut events: MessageReader<TextInputSubmitMessage>,
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
