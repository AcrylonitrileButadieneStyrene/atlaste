use std::sync::LazyLock;

use bevy::{
    feathers::{theme::ThemeBackgroundColor, tokens},
    prelude::*,
    ui_widgets::{Activate, observe},
};
use bevy_simple_text_input::TextInputSubmitMessage;

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
            Spawn(crate::components::text_input::new(
                "Search...",
                (
                    SearchMarker,
                    Node {
                        border: UiRect::bottom(Val::Px(1.)),
                        ..Default::default()
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
                observe(
                    |event: On<Pointer<Click>>, query: Query<&Entry>, mut commands: Commands| {
                        if event.button == PointerButton::Primary
                            && let Ok(entry) = query.get(event.original_event_target())
                        {
                            commands.trigger(EntryClicked(entry.id));
                        }
                    },
                ),
                observe(
                    |event: On<Pointer<Over>>,
                     mut commands: Commands,
                     filter: Query<Has<Entry>>| {
                        if filter
                            .get(event.original_event_target())
                            .unwrap_or_default()
                        {
                            commands
                                .entity(event.original_event_target())
                                .insert(ThemeBackgroundColor(tokens::BUTTON_BG_HOVER));
                        }
                    },
                ),
                observe(
                    |event: On<Pointer<Out>>, mut commands: Commands, filter: Query<Has<Entry>>| {
                        if filter
                            .get(event.original_event_target())
                            .unwrap_or_default()
                        {
                            // extracted because rustfmt messed it up inlined
                            type ComponentBundle = (ThemeBackgroundColor, BackgroundColor);
                            commands
                                .entity(event.original_event_target())
                                .remove::<ComponentBundle>();
                        }
                    },
                ),
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
            parent.spawn((
                Node {
                    margin: UiRect::left(Val::Px(entry.indentation as f32 * 8.0)),
                    ..Default::default()
                },
                Button,
                Text::default(),
                TextFont::from_font_size(if entry.id == 0 { 24. } else { 12. }),
                Pickable {
                    is_hoverable: true,
                    should_block_lower: false,
                },
                entry.clone(),
            ));
        }
    });

    commands.trigger(UpdateText(add.0));

    Ok(())
}

pub fn on_code_page_changed(code_page: On<VariantSelected<CodePage>>, mut commands: Commands) {
    commands.trigger(UpdateText(code_page.0));
}

pub static MAP_NAME_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::RegexBuilder::new(r"^(?:Map\d{4}\s*)?(.*?)(?:\s*P\d{0,4})?$")
        .case_insensitive(true)
        .build()
        .unwrap()
});

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
            let name = encoding.decode(&entry.name).0.to_string();
            if entry.id == 0 {
                text.0 = name;
            } else {
                let name = MAP_NAME_REGEX
                    .captures(&name)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                text.0 = format!("Map{:04} {name} P{}", entry.id, entry.parent);
            }
            text_font.font = font.clone();
        });
}
