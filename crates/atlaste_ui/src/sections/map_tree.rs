use bevy::{
    feathers::{theme::ThemeBackgroundColor, tokens},
    prelude::*,
    ui_widgets::observe,
};
use bevy_simple_text_input::{
    TextInput, TextInputCursorPos, TextInputInactive, TextInputPlaceholder, TextInputSettings,
    TextInputSubmitMessage,
};

use crate::{components::enum_selector::VariantSelected, sections::settings::CodePage};

#[derive(Component)]
pub struct ListMarker;

#[derive(Component)]
pub struct SearchMarker;

#[derive(Clone, Component)]
pub struct Entry {
    pub id: u32,
    pub parent: u32,
    pub name: Vec<u8>,
    pub indentation: u32,
}

#[derive(Event)]
pub struct AddEntries(pub CodePage, pub Vec<Entry>);

#[derive(Event)]
pub struct UpdateText(CodePage);

#[derive(Event)]
pub struct EntryClicked(pub u32);

pub fn new() -> impl Bundle {
    (
        Name::new("UI Map Tree"),
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            grid_row: GridPlacement::start(2),
            grid_column: GridPlacement::start(1),
            padding: UiRect::all(px(4)),
            overflow: Overflow::hidden_y(),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        Children::spawn((
            Spawn((
                SearchMarker,
                Node {
                    border: UiRect::bottom(Val::Px(1.)),
                    ..Default::default()
                },
                ThemeBackgroundColor(tokens::BUTTON_BG),
                TextInput,
                TextInputSettings {
                    retain_on_submit: true,
                    ..Default::default()
                },
                TextInputCursorPos::default(),
                TextInputInactive(true),
                TextInputPlaceholder {
                    value: String::from("Search..."),
                    ..Default::default()
                },
                observe(
                    |trigger: On<Pointer<Click>>, mut query: Query<&mut TextInputInactive>| {
                        query.get_mut(trigger.entity).unwrap().0 = false;
                    },
                ),
            )),
            Spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    ..Default::default()
                },
                ListMarker,
                ScrollPosition::DEFAULT,
            )),
        )),
    )
}

pub fn deselect_text_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut bevy_simple_text_input::TextInputInactive>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        query.par_iter_mut().for_each(|mut inactive| {
            inactive.0 = true;
        });
    }
}

pub fn search(
    mut events: MessageReader<TextInputSubmitMessage>,
    check: Query<Has<SearchMarker>>,
    mut query: Query<(&mut Node, &Text), With<Entry>>,
) -> Result {
    if let Some(event) = events
        .read()
        .filter(|event| check.get(event.entity).unwrap_or_default())
        .last()
    {
        query.par_iter_mut().for_each(|(mut node, text)| {
            node.display = if text.0.contains(&event.value) {
                Display::DEFAULT
            } else {
                Display::None
            };
        });
    };

    Ok(())
}

pub fn on_add_entries(
    add: On<AddEntries>,
    map_list: Query<Entity, With<ListMarker>>,
    mut commands: Commands,
) -> Result {
    let mut map_tree = commands.entity(map_list.single()?);
    map_tree.despawn_children();
    map_tree.with_children(|parent| {
        for entry in &add.1 {
            let id = entry.id;
            parent.spawn((
                Node {
                    margin: UiRect::left(Val::Px(entry.indentation as f32 * 8.0)),
                    ..Default::default()
                },
                Button,
                Text::default(),
                TextFont::from_font_size(12.),
                entry.clone(),
                Pickable {
                    is_hoverable: true,
                    should_block_lower: false,
                },
                observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(EntryClicked(id));
                }),
            ));
        }
    });

    commands.trigger(UpdateText(add.0));

    Ok(())
}

pub fn on_code_page_changed(code_page: On<VariantSelected<CodePage>>, mut commands: Commands) {
    commands.trigger(UpdateText(code_page.0));
}

pub fn on_update_text(
    update: On<UpdateText>,
    fonts: Res<crate::theme::fonts::Fonts>,
    mut entries: Query<(&mut Text, &mut TextFont, &Entry)>,
) {
    let encoding = update.0.to_encoding();
    let font = fonts.font(&update.0.to_theme_token());

    entries
        .par_iter_mut()
        .for_each(|(mut text, mut text_font, entry)| {
            // TODO: if the name doesn't include its map id, prepend it.
            // and also append the parent
            // example: Map0123 map name P0045
            // probably best to do with a regex
            text.0 = encoding.decode(&entry.name).0.to_string();
            text_font.font = font.clone();
        });
}
